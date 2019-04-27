use failure::Error;
use serde_derive::Deserialize;
use serde_json;
use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
pub struct Schema {
    annotations: Option<HashMap<String, String>>,
    endpoints: Vec<Endpoint>,
}

impl Schema {
    pub fn from_path(path: &str) -> Result<Self, Error> {
        let config_str = fs::read_to_string(path)?;
        let res: Schema = serde_json::from_str(&config_str)?;

        Ok(res)
    }
}


#[derive(Debug, Clone, Deserialize)]
pub struct Endpoint {
    name: String,
    sql_table: String,
    interface: Interface,
}

pub type KeyName = String;

#[derive(Debug, Clone, Deserialize)]
pub struct Interface(HashMap<KeyName, KeyConfig>);

#[derive(Debug, Clone, Deserialize)]
pub struct KeyConfig {
    column: Option<String>,
    filter_type: Option<FilterType>,
    visible: Option<bool>,
    dimension: Option<Dimension>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dimension {
    sql_table: Option<String>,
    parents: Interface,
}

#[derive(Debug, Clone, Deserialize)]
pub enum FilterType {
    #[serde(rename="numeric")]
    Numeric,
    #[serde(rename="string_match")]
    StringMatch,
    #[serde(rename="in_array")]
    InArray,
}

