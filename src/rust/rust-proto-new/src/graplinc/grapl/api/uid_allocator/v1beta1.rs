#[cfg(feature = "uid-allocator-server")]
pub mod server {
    use std::net::SocketAddr;

    use futures::FutureExt;
    use tokio::{
        net::TcpListener,
        sync::oneshot::Receiver,
    };
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::{
        transport::Server,
        Request,
        Response,
    };

    use crate::{
        graplinc::grapl::api::uid_allocator::v1beta1::messages::{
            AllocateIdsRequest,
            AllocateIdsResponse,
        },
        protobufs::graplinc::grapl::api::uid_allocator::v1beta1::{
            uid_allocator_server::{
                UidAllocator as UidAllocatorProto,
                UidAllocatorServer as UidAllocatorServerProto,
            },
            AllocateIdsRequest as AllocateIdsRequestProto,
            AllocateIdsResponse as AllocateIdsResponseProto,
        },
        protocol::{
            healthcheck::{
                server::init_health_service,
                HealthcheckStatus,
            },
            status::Status,
        },
    };

    #[async_trait::async_trait]
    pub trait UidAllocatorApi {
        type Error: Into<Status>;

        /// Requests a new allocation of Uids for a given tenant
        /// Note that it may not always return the requested size, but it will
        /// never return an empty allocation
        async fn allocate_ids(
            &self,
            request: AllocateIdsRequest,
        ) -> Result<AllocateIdsResponse, Self::Error>;
    }

    #[async_trait::async_trait]
    impl<T, E> UidAllocatorProto for T
    where
        T: UidAllocatorApi<Error = E> + Send + Sync + 'static,
        E: Into<Status> + Send + Sync + 'static,
    {
        async fn allocate_ids(
            &self,
            raw_request: Request<AllocateIdsRequestProto>,
        ) -> Result<Response<AllocateIdsResponseProto>, tonic::Status> {
            let proto_request = raw_request.into_inner();
            let request: AllocateIdsRequest = match proto_request.try_into() {
                Ok(request) => request,
                Err(e) => return Err(tonic::Status::invalid_argument(e.to_string())),
            };
            let response = UidAllocatorApi::allocate_ids(self, request)
                .await
                .map_err(|e| e.into())?;

            Ok(Response::new(response.into()))
        }
    }

    #[derive(thiserror::Error, Debug)]
    pub enum UidAllocatorServerError {
        #[error("grpc transport error: {0}")]
        GrpcTransportError(#[from] tonic::transport::Error),
        #[error("Bind error: {0}")]
        BindError(std::io::Error),
    }

    /// A server construct that drives the UidAllocatorApi implementation.
    pub struct UidAllocatorServer<T, E>
    where
        T: UidAllocatorApi<Error = E> + Send + Sync + 'static,
        E: Into<Status> + Send + Sync + 'static,
    {
        server: UidAllocatorServerProto<T>,
        addr: SocketAddr,
        shutdown_rx: Receiver<()>,
    }

    impl<T, E> UidAllocatorServer<T, E>
    where
        T: UidAllocatorApi<Error = E> + Send + Sync + 'static,
        E: Into<Status> + Send + Sync + 'static,
    {
        pub fn builder(
            service: T,
            addr: SocketAddr,
            shutdown_rx: Receiver<()>,
        ) -> UidAllocatorServerBuilder<T, E> {
            UidAllocatorServerBuilder::new(service, addr, shutdown_rx)
        }

        pub async fn serve(self) -> Result<(), UidAllocatorServerError> {
            let (healthcheck_handle, health_service) =
                init_health_service::<UidAllocatorServerProto<T>, _, _>(
                    || async { Ok(HealthcheckStatus::Serving) },
                    std::time::Duration::from_millis(500),
                )
                .await;

            let listener = TcpListener::bind(self.addr)
                .await
                .map_err(UidAllocatorServerError::BindError)?;

            Server::builder()
                .trace_fn(|request| {
                    tracing::trace_span!(
                        "UidAllocator",
                        headers = ?request.headers(),
                        method = ?request.method(),
                        uri = %request.uri(),
                        extensions = ?request.extensions(),
                    )
                })
                .add_service(health_service)
                .add_service(self.server)
                .serve_with_incoming_shutdown(
                    TcpListenerStream::new(listener),
                    self.shutdown_rx.map(|_| ()),
                )
                .then(|result| async move {
                    healthcheck_handle.abort();
                    result
                })
                .await?;
            Ok(())
        }
    }

    pub struct UidAllocatorServerBuilder<T, E>
    where
        T: UidAllocatorApi<Error = E> + Send + Sync + 'static,
        E: Into<Status> + Send + Sync + 'static,
    {
        server: UidAllocatorServerProto<T>,
        addr: SocketAddr,
        shutdown_rx: Receiver<()>,
    }

    impl<T, E> UidAllocatorServerBuilder<T, E>
    where
        T: UidAllocatorApi<Error = E> + Send + Sync + 'static,
        E: Into<Status> + Send + Sync + 'static,
    {
        /// Create a new builder for a UidAllocatorServer,
        /// taking the required arguments upfront.
        pub fn new(service: T, addr: SocketAddr, shutdown_rx: Receiver<()>) -> Self {
            Self {
                server: UidAllocatorServerProto::new(service),
                addr,
                shutdown_rx,
            }
        }

        /// Consumes the builder and returns a new `UidAllocatorServer`.
        /// Note: Panics on invalid build state
        pub fn build(self) -> UidAllocatorServer<T, E> {
            UidAllocatorServer {
                server: self.server,
                addr: self.addr,
                shutdown_rx: self.shutdown_rx,
            }
        }
    }
}

#[cfg(feature = "uid-allocator-client")]
pub mod client {
    use crate::{
        graplinc::grapl::api::uid_allocator::v1beta1::messages::{
            AllocateIdsRequest,
            AllocateIdsResponse,
        },
        protobufs::graplinc::grapl::api::uid_allocator::v1beta1::{
            uid_allocator_client::UidAllocatorClient as UidAllocatorClientProto,
            AllocateIdsRequest as AllocateIdsRequestProto,
        },
        protocol::status::Status,
        SerDeError,
    };

    #[derive(thiserror::Error, Debug)]
    pub enum UidAllocatorClientError {
        #[error("Failed to deserialize response {0}")]
        SerDeError(#[from] SerDeError),
        #[error("Status {0}")]
        Status(Status),
        #[error("ConnectError")]
        ConnectError(tonic::transport::Error),
    }

    #[derive(Clone)]
    /// This allocator should rarely be used. Instead, use the CachingUidAllocatorClient
    /// from the uid-allocator crate.
    pub struct UidAllocatorClient {
        inner: UidAllocatorClientProto<tonic::transport::Channel>,
    }

    impl UidAllocatorClient {
        pub async fn connect<T>(endpoint: T) -> Result<Self, UidAllocatorClientError>
        where
            T: std::convert::TryInto<tonic::transport::Endpoint>,
            T::Error: std::error::Error + Send + Sync + 'static,
        {
            Ok(UidAllocatorClient {
                inner: UidAllocatorClientProto::connect(endpoint)
                    .await
                    .map_err(UidAllocatorClientError::ConnectError)?,
            })
        }

        pub async fn allocate_ids(
            &mut self,
            request: AllocateIdsRequest,
        ) -> Result<AllocateIdsResponse, UidAllocatorClientError> {
            let raw_request: AllocateIdsRequestProto = request.into();
            let raw_response = self
                .inner
                .allocate_ids(raw_request)
                .await
                .map_err(|s| UidAllocatorClientError::Status(s.into()))?;
            let proto_response = raw_response.into_inner();
            let response = proto_response.try_into()?;
            Ok(response)
        }
    }
}

#[cfg(feature = "uid-allocator-messages")]
pub mod messages {
    use crate::{
        protobufs::graplinc::grapl::api::uid_allocator::v1beta1::{
            AllocateIdsRequest as AllocateIdsRequestProto,
            AllocateIdsResponse as AllocateIdsResponseProto,
            Allocation as AllocationProto,
        },
        serde_impl,
        type_url,
        SerDeError,
    };

    #[derive(Copy, Clone, Debug, PartialEq)]
    pub struct Allocation {
        pub start: u64,
        pub offset: u32,
    }

    impl TryFrom<AllocationProto> for Allocation {
        type Error = SerDeError;

        fn try_from(proto: AllocationProto) -> Result<Self, Self::Error> {
            Ok(Self {
                start: proto.start,
                offset: proto.offset,
            })
        }
    }

    impl From<Allocation> for AllocationProto {
        fn from(allocation: Allocation) -> Self {
            Self {
                start: allocation.start,
                offset: allocation.offset,
            }
        }
    }

    impl Iterator for Allocation {
        type Item = u64;

        fn next(&mut self) -> Option<Self::Item> {
            if self.start == (self.offset as u64) {
                None
            } else {
                let result = self.start;
                self.start += 1;
                Some(result)
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct AllocateIdsRequest {
        pub count: u32,
        pub tenant_id: uuid::Uuid,
    }

    impl TryFrom<AllocateIdsRequestProto> for AllocateIdsRequest {
        type Error = SerDeError;

        fn try_from(proto: AllocateIdsRequestProto) -> Result<Self, Self::Error> {
            let tenant_id = proto
                .tenant_id
                .ok_or_else(|| SerDeError::MissingField("tenant_id"))?
                .into();

            Ok(Self {
                count: proto.count,
                tenant_id,
            })
        }
    }

    impl From<AllocateIdsRequest> for AllocateIdsRequestProto {
        fn from(request: AllocateIdsRequest) -> Self {
            Self {
                count: request.count,
                tenant_id: Some(request.tenant_id.into()),
            }
        }
    }

    #[derive(Clone, Debug, PartialEq)]
    pub struct AllocateIdsResponse {
        pub allocation: Allocation,
    }

    impl TryFrom<AllocateIdsResponseProto> for AllocateIdsResponse {
        type Error = SerDeError;

        fn try_from(proto: AllocateIdsResponseProto) -> Result<Self, Self::Error> {
            let allocation = proto
                .allocation
                .ok_or_else(|| SerDeError::MissingField("allocation"))?
                .try_into()?;

            Ok(Self { allocation })
        }
    }

    impl From<AllocateIdsResponse> for AllocateIdsResponseProto {
        fn from(response: AllocateIdsResponse) -> Self {
            Self {
                allocation: Some(response.allocation.into()),
            }
        }
    }

    impl serde_impl::ProtobufSerializable<Allocation> for Allocation {
        type ProtobufMessage = AllocationProto;
    }

    impl serde_impl::ProtobufSerializable<AllocateIdsRequest> for AllocateIdsRequest {
        type ProtobufMessage = AllocateIdsRequestProto;
    }

    impl serde_impl::ProtobufSerializable<AllocateIdsResponse> for AllocateIdsResponse {
        type ProtobufMessage = AllocateIdsResponseProto;
    }

    impl type_url::TypeUrl for Allocation {
        const TYPE_URL: &'static str = "graplinc.grapl.api.uid_allocator.v1beta1.Allocation";
    }

    impl type_url::TypeUrl for AllocateIdsRequest {
        const TYPE_URL: &'static str =
            "graplinc.grapl.api.uid_allocator.v1beta1.AllocateIdsRequest";
    }

    impl type_url::TypeUrl for AllocateIdsResponse {
        const TYPE_URL: &'static str =
            "graplinc.grapl.api.uid_allocator.v1beta1.AllocateIdsResponse";
    }
}
