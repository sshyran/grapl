use std::{
    fmt::Debug,
    time::Duration,
};

use bytes::Bytes;
use client_executor::{
    strategy::jitter,
    Executor,
    ExecutorConfig,
};
use futures::{
    Stream,
    StreamExt,
};
use proto::plugin_registry_service_client::PluginRegistryServiceClient as PluginRegistryServiceClientProto;

use crate::{
    graplinc::grapl::api::plugin_registry::v1beta1 as native,
    protobufs::graplinc::grapl::api::plugin_registry::v1beta1 as proto,
    protocol::{
        endpoint::Endpoint,
        service_client::{
            ConnectError,
            Connectable,
        },
        status::Status,
    },
    SerDeError,
};

#[derive(Debug, thiserror::Error)]
pub enum PluginRegistryServiceClientError {
    #[error("ErrorStatus {0}")]
    ErrorStatus(#[from] Status),
    #[error("PluginRegistryDeserializationError {0}")]
    PluginRegistryDeserializationError(#[from] SerDeError),
    #[error("CircuitOpen")]
    CircuitOpen,
    #[error("Timeout")]
    Elapsed,
}

impl From<client_executor::Error<tonic::Status>> for PluginRegistryServiceClientError {
    fn from(e: client_executor::Error<tonic::Status>) -> Self {
        match e {
            client_executor::Error::Inner(e) => Self::ErrorStatus(e.into()),
            client_executor::Error::Rejected => Self::CircuitOpen,
            client_executor::Error::Elapsed(_) => Self::Elapsed,
        }
    }
}

#[derive(Clone)]
pub struct PluginRegistryServiceClient {
    proto_client: PluginRegistryServiceClientProto<tonic::transport::Channel>,
    executor: Executor,
}

#[async_trait::async_trait]
impl Connectable for PluginRegistryServiceClient {
    const SERVICE_NAME: &'static str =
        "graplinc.grapl.api.plugin_registry.v1beta1.PluginRegistryService";

    #[tracing::instrument(err)]
    async fn connect(endpoint: Endpoint) -> Result<Self, ConnectError> {
        let executor = Executor::new(ExecutorConfig::new(Duration::from_secs(30)));
        let proto_client = executor
            .spawn(
                client_executor::strategy::FibonacciBackoff::from_millis(10)
                    .map(jitter)
                    .take(20),
                || {
                    let endpoint = endpoint.clone();
                    async move {
                        PluginRegistryServiceClientProto::connect(endpoint)
                            .await
                            .map_err(ConnectError::from)
                    }
                },
            )
            .await?;

        Ok(PluginRegistryServiceClient {
            proto_client,
            executor,
        })
    }
}

impl PluginRegistryServiceClient {
    /// create a new plugin.
    /// NOTE: Most consumers will want `create_plugin`, not `create_plugin_raw`.
    pub async fn create_plugin_raw<S>(
        &mut self,
        request: S,
    ) -> Result<native::CreatePluginResponse, PluginRegistryServiceClientError>
    where
        S: Stream<Item = native::CreatePluginRequest> + Send + 'static,
    {
        let response = match self
            .proto_client
            .create_plugin(request.map(proto::CreatePluginRequest::from))
            .await
        {
            Ok(r) => r,
            Err(e) => return Err(Status::from(e).into()),
        };
        let response = native::CreatePluginResponse::try_from(response.into_inner())?;
        Ok(response)
    }

    /// Create a new plugin
    ///
    pub async fn create_plugin<S>(
        &mut self,
        metadata: native::PluginMetadata,
        plugin_artifact: S,
    ) -> Result<native::CreatePluginResponse, PluginRegistryServiceClientError>
    where
        S: Stream<Item = Bytes> + Send + 'static,
    {
        // Send the metadata first followed by N chunks
        let request = futures::stream::iter(std::iter::once(
            native::CreatePluginRequest::Metadata(metadata),
        ))
        .chain(plugin_artifact.map(native::CreatePluginRequest::Chunk));

        self.create_plugin_raw(request).await
    }

    /// retrieve the plugin corresponding to the given plugin_id
    pub async fn get_plugin(
        &mut self,
        request: native::GetPluginRequest,
    ) -> Result<native::GetPluginResponse, PluginRegistryServiceClientError> {
        let response = self
            .executor
            .spawn(
                client_executor::strategy::FibonacciBackoff::from_millis(10)
                    .map(jitter)
                    .take(20),
                || {
                    let mut proto_client = self.proto_client.clone();
                    let request = request.clone();
                    async move {
                        proto_client
                            .get_plugin(proto::GetPluginRequest::from(request))
                            .await
                    }
                },
            )
            .await?;

        let response = native::GetPluginResponse::try_from(response.into_inner())?;
        Ok(response)
    }

    /// turn on a particular plugin's code
    pub async fn deploy_plugin(
        &mut self,
        request: native::DeployPluginRequest,
    ) -> Result<native::DeployPluginResponse, PluginRegistryServiceClientError> {
        let response = self
            .executor
            .spawn(
                client_executor::strategy::FibonacciBackoff::from_millis(10)
                    .map(jitter)
                    .take(20),
                || {
                    let mut proto_client = self.proto_client.clone();
                    let request = request.clone();
                    async move {
                        proto_client
                            .deploy_plugin(proto::DeployPluginRequest::from(request))
                            .await
                    }
                },
            )
            .await?;
        let response = native::DeployPluginResponse::try_from(response.into_inner())?;
        Ok(response)
    }

    /// turn off a particular plugin's code
    pub async fn tear_down_plugin(
        &mut self,
        request: native::TearDownPluginRequest,
    ) -> Result<native::TearDownPluginResponse, PluginRegistryServiceClientError> {
        self.proto_client
            .tear_down_plugin(proto::TearDownPluginRequest::from(request))
            .await
            .map_err(Status::from)?;
        todo!()
    }

    pub async fn get_plugin_health(
        &mut self,
        request: native::GetPluginHealthRequest,
    ) -> Result<native::GetPluginHealthResponse, PluginRegistryServiceClientError> {
        let response = self
            .executor
            .spawn(
                client_executor::strategy::FibonacciBackoff::from_millis(10)
                    .map(jitter)
                    .take(20),
                || {
                    let mut proto_client = self.proto_client.clone();
                    let request = request.clone();
                    async move {
                        proto_client
                            .get_plugin_health(proto::GetPluginHealthRequest::from(request))
                            .await
                    }
                },
            )
            .await?;
        let response = native::GetPluginHealthResponse::try_from(response.into_inner())?;
        Ok(response)
    }

    /// Given information about an event source, return all generators that handle that event source
    #[tracing::instrument(skip(self, request), err)]
    pub async fn get_generators_for_event_source(
        &mut self,
        request: native::GetGeneratorsForEventSourceRequest,
    ) -> Result<native::GetGeneratorsForEventSourceResponse, PluginRegistryServiceClientError> {
        let response = self
            .executor
            .spawn(
                client_executor::strategy::FibonacciBackoff::from_millis(10)
                    .map(jitter)
                    .take(20),
                || {
                    let mut proto_client = self.proto_client.clone();
                    let request = request.clone();
                    async move {
                        proto_client
                            .get_generators_for_event_source(
                                proto::GetGeneratorsForEventSourceRequest::from(request),
                            )
                            .await
                    }
                },
            )
            .await?;
        let response = native::GetGeneratorsForEventSourceResponse::from(response.into_inner());

        Ok(response)
    }

    /// Given information about a tenant, return all analyzers for that tenant
    pub async fn get_analyzers_for_tenant(
        &mut self,
        request: native::GetAnalyzersForTenantRequest,
    ) -> Result<native::GetAnalyzersForTenantResponse, PluginRegistryServiceClientError> {
        self.proto_client
            .get_analyzers_for_tenant(proto::GetAnalyzersForTenantRequest::from(request))
            .await
            .map_err(Status::from)?;
        todo!()
    }
}
