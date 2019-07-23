mod schema_config;

use failure::{Error, bail, format_err};
use indexmap::IndexMap;
use serde_derive::{Deserialize, Serialize};
use serde_json;
use std::convert::From;
use std::fs;
use std::sync::{Arc, RwLock};
use tera::{Tera, Context};

use schema_config::*;
use crate::middleware::X_BERYL_SECRET;
use crate::query::Query;
use crate::query_ir::{
    QueryIr,
    FilterIr,
    SortIr,
};

#[derive(Debug, Clone)]
pub struct Schema {
    pub annotations: IndexMap<String, String>,
    pub endpoints: Vec<Endpoint>,
}

impl Schema {
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(path)?;
        let schema_config: SchemaConfig = serde_json::from_str(&config_str)?;

        Ok(schema_config.into())
    }
    pub fn get_endpoint(&self, endpoint_path: &str) -> Option<Endpoint> {
        self.endpoints.iter()
            .find(|e| e.name == endpoint_path)
            .cloned()
    }

    pub fn gen_query_ir(
        &self,
        endpoint: &str,
        query: &Query,
        sql_templates: &Option<Arc<RwLock<Tera>>>,
        ) -> Result<(QueryIr, Vec<String>), Error>
    {
        // checks?

        // query_ir
        // =========================================
        let schema_endpoint = self.endpoints
            .iter()
            .find(|ept| ept.name == endpoint)
            .ok_or_else(|| format_err!("Couldn't find endpoint in schema"))?;

        let table = match schema_endpoint.sql_select {
            SqlSelect::Table { ref name } => name.clone(),
            SqlSelect::Template { ref template_path } => {
                let mut context = Context::new();

                // TODO make this cached
                // this is template vars that are specified in the endpoint
                let template_vars: Vec<_> = schema_endpoint.interface.0.iter()
                    .filter_map(|(key, param_value)| {
                        if param_value.is_template_var {
                            Some(key)
                        } else {
                            None
                        }
                    })
                    .collect();

                let context_vars = query.filters.iter()
                    .filter_map(|(key, filter_member)| {
                        if template_vars.contains(&key) {
                            Some((key, filter_member))
                        } else {
                            None
                        }
                    });


                for (k, v) in context_vars {
                    context.insert(&k, &v);
                }

                if let Some(tera) = sql_templates {
                    tera.read()
                        .expect("poison lock on tera")
                        .render(&template_path, &context)
                        .map_err(|e| format_err!("{}", e))?
                } else {
                    bail!("Could not render sql template");
                }
            }
        };

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

        // filters are different, they need to also be parsed
        // based on schematype here.
        let filters: Result<_, Error> = query.filters.iter()
            .filter(|(name, _)| {
                name != &X_BERYL_SECRET
            })
            .map(|(name, filter_query)| {
                FilterIr::from_schema_query(name, filter_query, &schema_endpoint.interface)
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
    pub name: String,
    pub sql_select: SqlSelect,
    pub primary:Option<String>,
    pub interface: Interface,
}

#[derive(Debug, Clone)]
pub struct Interface(pub IndexMap<ParamKey, ParamValue>);

#[derive(Debug, Clone)]
pub struct ParamValue {
    pub column: String,
    pub filter_type: FilterType,
    pub visible: bool,
    pub dimension: Option<Dimension>,
    pub is_text: bool,
    pub is_template_var: bool,
}

#[derive(Debug, Clone)]
pub struct Dimension{
    sql_table: String,
    parents: Interface,
}

pub type ParamKey = String;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum FilterType {
    #[serde(rename="compare")]
    Compare,
    #[serde(rename="exact_match")]
    ExactMatch,
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
            sql_select: config.sql_select.into(),
            primary: config.primary,
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
                     is_template_var: p_config.is_template_var.unwrap_or(false),
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

#[derive(Debug, Clone, PartialEq)]
pub enum SqlSelect{
    Table {
        name: String,
    },
    Template {
        template_path: String,
    },
}

impl From<SqlSelectConfig> for SqlSelect {
    fn from(config: SqlSelectConfig) -> Self {
        match config {
            SqlSelectConfig::Table { name } => {
                SqlSelect::Table { name }
            },
            SqlSelectConfig::Template { template_path } => {
                SqlSelect::Template { template_path}
            },

        }

    }
}
