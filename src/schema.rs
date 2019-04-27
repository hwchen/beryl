mod schema_config;

use failure::Error;
use indexmap::IndexMap;
use serde_derive::Deserialize;
use serde_json;
use std::convert::From;
use std::fs;

use schema_config::*;

#[derive(Debug, Clone)]
pub struct Schema {
    annotations: IndexMap<String, String>,
    endpoints: Vec<Endpoint>,
}

impl Schema {
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(path)?;
        let schema_config: SchemaConfig = serde_json::from_str(&config_str)?;

        Ok(schema_config.into())
    }
}
#[derive(Debug, Clone)]
pub struct Endpoint{
    name: String,
    sql_table: String,
    interface: Interface,
}

#[derive(Debug, Clone)]
pub struct Interface(IndexMap<ParamKey, ParamValue>);

#[derive(Debug, Clone)]
pub struct ParamValue {
    column: String,
    filter_type: FilterType,
    visible: bool,
    dimension: Option<Dimension>,
}

#[derive(Debug, Clone)]
pub struct Dimension{
    sql_table: String,
    parents: Interface,
}

pub type ParamKey = String;

#[derive(Debug, Clone, Deserialize)]
pub enum FilterType {
    #[serde(rename="numeric")]
    Numeric,
    #[serde(rename="string_match")]
    StringMatch,
    #[serde(rename="in_array")]
    InArray,
}

impl From<SchemaConfig> for Schema {
    fn from(config: SchemaConfig) -> Self {
        Schema {
            annotations: config.annotations.unwrap_or_else(|| IndexMap::new()),
            endpoints: config.endpoints.iter().map(|e| e.clone().into()).collect(),
        }
    }
}

impl From<EndpointConfig> for Endpoint {
    fn from(config: EndpointConfig) -> Self {
        Endpoint {
            name: config.name,
            sql_table: config.sql_table,
            interface: config.interface.into(),
        }
    }
}

/// This is where defaults for Interface are set.
impl From<InterfaceConfig> for Interface {
    fn from(config: InterfaceConfig) -> Self {
        let res = config.0.iter()
            .map(|(param_key, p_config)| {
                (param_key.clone(),
                 ParamValue {
                     column: p_config.column.clone().unwrap_or(param_key.to_owned()),
                     filter_type: p_config.filter_type.clone().unwrap_or(FilterType::Numeric),
                     visible: p_config.visible.unwrap_or(true),
                     dimension: p_config.dimension.clone().map(|d| d.into()),
                 },
                )
            }).collect();

        Interface(res)
    }
}

impl From<DimensionConfig> for Dimension {
    fn from(config: DimensionConfig) -> Self {
        Dimension {
            sql_table: config.sql_table,
            parents: config.parents.into(),
        }
    }
}
