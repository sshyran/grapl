use bytes::Bytes;

use crate::{
    graplinc::grapl::common::v1beta1::types::{
        EdgeName,
        NodeType,
    },
    protobufs::graplinc::grapl::api::graph_schema_manager::v1beta1::{
        DeploySchemaRequest as DeploySchemaRequestProto,
        DeploySchemaResponse as DeploySchemaResponseProto,
        EdgeCardinality as EdgeCardinalityProto,
        GetEdgeSchemaRequest as GetEdgeSchemaRequestProto,
        GetEdgeSchemaResponse as GetEdgeSchemaResponseProto,
        SchemaType as SchemaTypeProto,
    },
    serde_impl,
    type_url,
    SerDeError,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SchemaType {
    GraphqlV0,
}

impl TryFrom<SchemaTypeProto> for SchemaType {
    type Error = SerDeError;

    fn try_from(response_proto: SchemaTypeProto) -> Result<Self, Self::Error> {
        match response_proto {
            SchemaTypeProto::GraphqlV0 => Ok(SchemaType::GraphqlV0),
            SchemaTypeProto::Unspecified => Err(SerDeError::UnknownVariant("SchemaType")),
        }
    }
}

impl From<SchemaType> for SchemaTypeProto {
    fn from(response: SchemaType) -> Self {
        match response {
            SchemaType::GraphqlV0 => SchemaTypeProto::GraphqlV0,
        }
    }
}

impl type_url::TypeUrl for SchemaType {
    const TYPE_URL: &'static str =
        "graplsecurity.com/graplinc.grapl.api.graph_schema_manager.v1beta1.SchemaType";
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeploySchemaRequest {
    pub tenant_id: uuid::Uuid,
    pub schema: Bytes,
    pub schema_type: SchemaType,
    pub schema_version: u32,
}

impl TryFrom<DeploySchemaRequestProto> for DeploySchemaRequest {
    type Error = SerDeError;

    fn try_from(value: DeploySchemaRequestProto) -> Result<Self, Self::Error> {
        let schema_type = value.schema_type().try_into()?;
        let schema = value.schema;
        if schema.is_empty() {
            return Err(SerDeError::InvalidField {
                field_name: "schema",
                assertion: "must not be empty".to_owned(),
            });
        }

        let tenant_id = value
            .tenant_id
            .ok_or(SerDeError::MissingField("DeploySchemaRequest.tenant_id"))?
            .into();

        Ok(DeploySchemaRequest {
            tenant_id,
            schema,
            schema_type,
            schema_version: value.schema_version,
        })
    }
}

impl From<DeploySchemaRequest> for DeploySchemaRequestProto {
    fn from(value: DeploySchemaRequest) -> Self {
        let schema_type: SchemaTypeProto = value.schema_type.into();
        DeploySchemaRequestProto {
            tenant_id: Some(value.tenant_id.into()),
            schema_type: schema_type as i32,
            schema: value.schema,
            schema_version: value.schema_version,
        }
    }
}

impl type_url::TypeUrl for DeploySchemaRequest {
    const TYPE_URL: &'static str =
        "graplsecurity.com/graplinc.grapl.api.graph_schema_manager.v1beta1.DeploySchemaRequest";
}

impl serde_impl::ProtobufSerializable for DeploySchemaRequest {
    type ProtobufMessage = DeploySchemaRequestProto;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DeploySchemaResponse {}

impl TryFrom<DeploySchemaResponseProto> for DeploySchemaResponse {
    type Error = SerDeError;

    fn try_from(response_proto: DeploySchemaResponseProto) -> Result<Self, Self::Error> {
        let DeploySchemaResponseProto {} = response_proto;
        Ok(DeploySchemaResponse {})
    }
}

impl From<DeploySchemaResponse> for DeploySchemaResponseProto {
    fn from(response: DeploySchemaResponse) -> Self {
        let DeploySchemaResponse {} = response;
        DeploySchemaResponseProto {}
    }
}

impl type_url::TypeUrl for DeploySchemaResponse {
    const TYPE_URL: &'static str =
        "graplsecurity.com/graplinc.grapl.api.graph_schema_manager.v1beta1.DeploySchemaResponse";
}

impl serde_impl::ProtobufSerializable for DeploySchemaResponse {
    type ProtobufMessage = DeploySchemaResponseProto;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetEdgeSchemaRequest {
    pub tenant_id: uuid::Uuid,
    pub node_type: NodeType,
    pub edge_name: EdgeName,
}

impl TryFrom<GetEdgeSchemaRequestProto> for GetEdgeSchemaRequest {
    type Error = SerDeError;

    fn try_from(response_proto: GetEdgeSchemaRequestProto) -> Result<Self, Self::Error> {
        let tenant_id = response_proto
            .tenant_id
            .ok_or(SerDeError::MissingField("GetEdgeSchemaRequest.tenant_id"))?
            .into();

        let node_type = response_proto
            .node_type
            .ok_or(SerDeError::MissingField("GetEdgeSchemaRequest.node_type"))?
            .try_into()?;

        let edge_name = response_proto
            .edge_name
            .ok_or(SerDeError::MissingField("GetEdgeSchemaRequest.edge_name"))?
            .try_into()?;

        Ok(GetEdgeSchemaRequest {
            tenant_id,
            node_type,
            edge_name,
        })
    }
}

impl From<GetEdgeSchemaRequest> for GetEdgeSchemaRequestProto {
    fn from(response: GetEdgeSchemaRequest) -> Self {
        GetEdgeSchemaRequestProto {
            tenant_id: Some(response.tenant_id.into()),
            node_type: Some(response.node_type.into()),
            edge_name: Some(response.edge_name.into()),
        }
    }
}

impl type_url::TypeUrl for GetEdgeSchemaRequest {
    const TYPE_URL: &'static str =
        "graplsecurity.com/graplinc.grapl.api.graph_schema_manager.v1beta1.GetEdgeSchemaRequest";
}

impl serde_impl::ProtobufSerializable for GetEdgeSchemaRequest {
    type ProtobufMessage = GetEdgeSchemaRequestProto;
}

#[derive(Debug, Clone, PartialEq)]
pub struct GetEdgeSchemaResponse {
    pub reverse_edge_name: EdgeName,
    pub cardinality: EdgeCardinality,
    pub reverse_cardinality: EdgeCardinality,
}

impl TryFrom<GetEdgeSchemaResponseProto> for GetEdgeSchemaResponse {
    type Error = SerDeError;

    fn try_from(response_proto: GetEdgeSchemaResponseProto) -> Result<Self, Self::Error> {
        let cardinality = response_proto.cardinality().try_into()?;
        let reverse_cardinality = response_proto.reverse_cardinality().try_into()?;

        let reverse_edge_name = response_proto
            .reverse_edge_name
            .ok_or(SerDeError::MissingField(
                "GetEdgeSchemaResponse.reverse_edge_name",
            ))?
            .try_into()?;

        Ok(GetEdgeSchemaResponse {
            reverse_edge_name,
            cardinality,
            reverse_cardinality,
        })
    }
}

impl From<GetEdgeSchemaResponse> for GetEdgeSchemaResponseProto {
    fn from(value: GetEdgeSchemaResponse) -> Self {
        let cardinality: EdgeCardinalityProto = value.cardinality.into();
        let reverse_cardinality: EdgeCardinalityProto = value.reverse_cardinality.into();
        GetEdgeSchemaResponseProto {
            reverse_edge_name: Some(value.reverse_edge_name.into()),
            cardinality: cardinality as i32,
            reverse_cardinality: reverse_cardinality as i32,
        }
    }
}

impl type_url::TypeUrl for GetEdgeSchemaResponse {
    const TYPE_URL: &'static str =
        "graplsecurity.com/graplinc.grapl.api.graph_schema_manager.v1beta1.GetEdgeSchemaResponse";
}

impl serde_impl::ProtobufSerializable for GetEdgeSchemaResponse {
    type ProtobufMessage = GetEdgeSchemaResponseProto;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EdgeCardinality {
    ToOne,
    ToMany,
}

impl TryFrom<EdgeCardinalityProto> for EdgeCardinality {
    type Error = SerDeError;

    fn try_from(response_proto: EdgeCardinalityProto) -> Result<Self, Self::Error> {
        match response_proto {
            EdgeCardinalityProto::ToOne => Ok(EdgeCardinality::ToOne),
            EdgeCardinalityProto::ToMany => Ok(EdgeCardinality::ToMany),
            EdgeCardinalityProto::Unspecified => Err(SerDeError::UnknownVariant("EdgeCardinality")),
        }
    }
}

impl From<EdgeCardinality> for EdgeCardinalityProto {
    fn from(response: EdgeCardinality) -> Self {
        match response {
            EdgeCardinality::ToOne => EdgeCardinalityProto::ToOne,
            EdgeCardinality::ToMany => EdgeCardinalityProto::ToMany,
        }
    }
}

impl type_url::TypeUrl for EdgeCardinality {
    const TYPE_URL: &'static str =
        "graplsecurity.com/graplinc.grapl.api.graph_schema_manager.v1beta1.EdgeCardinality";
}
