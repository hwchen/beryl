//! Query is the fully typed representation of a query.
//!
//! an ApiQueryOpt from the api handler has the same query
//! param structure, but has only strings for the param values.
//!
//! in the api handler, From<ApiQueryOpt> for Query is implemented.
//!
//! Then the next step is going from a Query to a QueryIr, which
//! is a typed representation of the information needed to
//! directly produce a sql statement.
//!

use failure::{Error, bail};
use indexmap::IndexMap;
use std::str::FromStr;

pub struct Query {
    pub filters: FiltersQuery,
    pub sort: Option<SortQuery>,
    pub limit: Option<LimitQuery>,
}

pub type FiltersQuery = IndexMap<String, FilterQuery>;

pub type FilterQuery = String;

#[derive(Debug, Clone)]
pub struct LimitQuery {
    pub offset: Option<u64>,
    pub n: u64,
}

impl FromStr for LimitQuery {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(",").collect::<Vec<_>>()[..] {
            [offset, n] => {
                Ok(LimitQuery {
                    offset: Some(offset.parse::<u64>()?),
                    n: n.parse::<u64>()?,
                })
            },
            [n] => {
                Ok(LimitQuery {
                    offset: None,
                    n: n.parse::<u64>()?,
                })
            },
            _ => bail!("Could not parse a limit query"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SortQuery {
    pub direction: SortDirection,
    pub name: String,
}

impl FromStr for SortQuery {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(".").collect::<Vec<_>>()[..] {
            [name, direction] => {
                let name = name.to_string();
                let direction = direction.parse::<SortDirection>()?;
                Ok(SortQuery {
                    direction,
                    name,
                })
            },
            _ => bail!("Could not parse a sort query"),
        }

    }
}

#[derive(Debug, Clone)]
pub enum SortDirection {
    Asc,
    Desc,
}

impl SortDirection {
    pub fn sql_string(&self) -> String {
        match *self {
            SortDirection::Asc => "asc".to_owned(),
            SortDirection::Desc => "desc".to_owned(),
        }
    }
}

impl FromStr for SortDirection {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "asc" => SortDirection::Asc,
            "desc" => SortDirection::Desc,
            _ => bail!("Could not parse sort direction"),
        })
    }
}

