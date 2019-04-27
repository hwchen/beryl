use serde_derive::Deserialize;
use indexmap::IndexMap;

use super::{
    ParamKey,
    FilterType,
};

#[derive(Debug, Clone, Deserialize)]
pub struct SchemaConfig {
    pub annotations: Option<IndexMap<String, String>>,
    pub endpoints: Vec<EndpointConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EndpointConfig {
    pub name: String,
    pub sql_table: String,
    pub interface: InterfaceConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct InterfaceConfig(pub IndexMap<ParamKey, ParamValueConfig>);

#[derive(Debug, Clone, Deserialize)]
pub struct ParamValueConfig {
    pub column: Option<String>,
    pub filter_type: Option<FilterType>,
    pub visible: Option<bool>,
    pub dimension: Option<DimensionConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DimensionConfig {
    pub sql_table: String,
    pub parents: InterfaceConfig,
}

