mod schema_config;

use failure::{Error, format_err};
use indexmap::IndexMap;
use serde_derive::Deserialize;
use serde_json;
use std::convert::From;
use std::fs;

use schema_config::*;

use crate::query::Query;
use crate::query_ir::{
    QueryIr,
    FilterIr,
    SortIr,
};

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

    pub fn gen_query_ir(
        &self,
        endpoint: &str,
        query: &Query
        ) -> Result<(QueryIr, Vec<String>), Error>
    {
        // checks?

        // query_ir
        let schema_endpoint = self.endpoints
            .iter()
            .find(|ept| ept.name == endpoint)
            .ok_or_else(|| format_err!("Couldn't find endpoint in schema"))?;

        let table = schema_endpoint.sql_table.clone();

        // projection is all interface names where visible is true
        let projection = schema_endpoint.interface.0.iter()
            .filter(|(_, param_value)| {
                param_value.visible
            })
            .map(|(_, param_value)| {
                param_value.column.clone()
            })
            .collect();

        // headers
        let headers = schema_endpoint.interface.0.iter()
            .filter(|(_, param_value)| {
                param_value.visible
            })
            .map(|(param_key, _)| {
                param_key.clone()
            })
            .collect();

        let filters: Result<_, Error> = query.filters.0.iter()
            .map(|(name, filter_query)| {
                // name of query should match with
                // name in interface
                let column_and_is_text: Result<_, Error> = schema_endpoint
                    .interface
                    .0.get(name)
                    .map(|interface_param_value| {
                        (interface_param_value.column.clone(),
                         interface_param_value.is_text.clone()
                        )
                    })
                    .ok_or_else(|| format_err!("query filter name not in schema"));

                let (column, is_text) = column_and_is_text?;

                Ok(FilterIr {
                    column,
                    constraint: filter_query.constraint.clone(),
                    is_text,
                })
            })
            .collect();
        let filters = filters?;

        let sort = if let Some(ref s) = query.sort {
            let column = schema_endpoint
                .interface
                .0.get(&s.name)
                .map(|interface_param_value| {
                    interface_param_value.column.clone()
                })
                .ok_or_else(|| format_err!("query filter name not in schema"))?;

            Some(SortIr {
                direction: s.direction.clone(),
                column,
            })
        } else {
            None
        };

        Ok((
            QueryIr{
                table,
                projection,
                filters,
                sort,
                limit: query.limit.clone(),
            },
            headers
        ))
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
    is_text: bool,
}

#[derive(Debug, Clone)]
pub struct Dimension{
    sql_table: String,
    parents: Interface,
}

pub type ParamKey = String;

#[derive(Debug, Clone, Deserialize)]
pub enum FilterType {
    #[serde(rename="compare")]
    Compare,
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
                     filter_type: p_config.filter_type.clone().unwrap_or(FilterType::Compare),
                     visible: p_config.visible.unwrap_or(true),
                     dimension: p_config.dimension.clone().map(|d| d.into()),
                     is_text: p_config.is_text.unwrap_or(false),
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
