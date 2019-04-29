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

pub struct FiltersQuery(pub IndexMap<String, FilterQuery>);

#[derive(Debug, Clone)]
pub struct FilterQuery {
    pub name: String,
    pub constraint: Constraint,
}

impl FromStr for FilterQuery {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(",").collect::<Vec<_>>()[..] {
            [name, constraint] => {
                let name = name.to_string();
                let constraint = constraint.parse::<Constraint>()?;

                Ok(FilterQuery {
                    name,
                    constraint,
                })
            },
            _ => bail!("Could not parse a filter query"),
        }
    }
}
// Constraint: less than, greater than a number
// This is a little less straightforward, so we should
// probably test this
#[derive(Debug, Clone)]
pub struct Constraint {
    pub comparison: Comparison,
    pub n: i64,
}

impl Constraint {
    pub fn sql_string(&self) -> String {
        format!("{} {}", self.comparison.sql_string(), self.n)
    }
}

impl FromStr for Constraint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(".").collect::<Vec<_>>()[..] {
            [comparison, n] => {

                let comparison = comparison.parse::<Comparison>()?;
                let n = n.parse::<i64>()?;

                Ok(Constraint {
                    comparison,
                    n,
                })
            },
            _ => bail!("Could not parse a Constraint"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Comparison {
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
}

impl Comparison {
    pub fn sql_string(&self) -> String {
        match self {
            Comparison::Equal => "=".to_owned(),
            Comparison::NotEqual => "<>".to_owned(),
            Comparison::LessThan => "<".to_owned(),
            Comparison::LessThanOrEqual => "<=".to_owned(),
            Comparison::GreaterThan => ">".to_owned(),
            Comparison::GreaterThanOrEqual => ">=".to_owned(),
        }
    }
}

impl FromStr for Comparison {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "eq" => Ok(Comparison::Equal),
            "neq" => Ok(Comparison::NotEqual),
            "lt" => Ok(Comparison::LessThan),
            "lte" => Ok(Comparison::LessThanOrEqual),
            "gt" => Ok(Comparison::GreaterThan),
            "gte" => Ok(Comparison::GreaterThanOrEqual),
            _ => bail!("Could not parse a comparison operator"),
        }
    }
}

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

