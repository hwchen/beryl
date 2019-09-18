use failure::{Error, format_err, bail};
use itertools::join;
use std::str::FromStr;

use crate::schema::{Interface, FilterType};

#[derive(Debug, Clone)]
pub struct FilterIr {
    pub column: String,
    pub constraint: Constraint,
    pub is_text: bool,
}

impl FilterIr {
    pub fn from_schema_query(name: &str, filter_query: &str, interface: &Interface) -> Result<Self, Error> {
        // first, get the param key and value
        let interface_param_value = interface.0.get(name)
            .ok_or_else(|| format_err!("No endpoint param found for filter '{}'", name))?;
        let column = interface_param_value.column.clone();
        let is_text = interface_param_value.is_text;
        let filter_type = &interface_param_value.filter_type;


        let constraint = match filter_type {
            FilterType::Compare => {
                // allow multiple comparisons, separated by commas
                let comparisons = filter_query.split(",")
                    .map(|one_comparison_str| {
                        match &one_comparison_str.split(".").collect::<Vec<_>>()[..] {
                            [constraint_type, members @ ..] => {
                                let comparison = constraint_type.parse::<Comparison>()?;
                                let n = join(members, "."); // doesn't check for malformed

                                Ok(Compare {
                                    comparison,
                                    n,
                                })
                            },
                            _ => Err(format_err!("Could not parse a Comparison for filter {}", name)),
                        }
                    })
                    .collect::<Result<Vec<_>,_>>()?;

                Constraint::CompareList(comparisons)
            },
            FilterType::ExactMatch => {
                Constraint::ExactMatch {
                    pattern: filter_query.to_owned(),
                }
            },
            FilterType::StringMatch => {
                Constraint::StringMatch {
                    substring: filter_query.to_owned(),
                }
            },
            FilterType::InArray => {
                let mut in_members = vec![];
                let mut not_in_members = vec![];

                for member in filter_query.split(",") {
                    let leading_char = member.chars()
                        .nth(0)
                        .ok_or(format_err!("blank member not allowed"))?;
                    println!("{}", leading_char);

                    if leading_char == '~' {
                        let stripped_member = member.chars()
                            .skip(1)
                            .collect();
                        not_in_members.push(stripped_member);
                    } else {
                        in_members.push(member.to_string());
                    }
                }

                Constraint::InArray {
                    in_members,
                    not_in_members,
                }
            }
        };

        Ok(FilterIr {
            column,
            constraint,
            is_text,
        })
    }
}
#[derive(Debug, Clone)]
pub enum Constraint {
    CompareList(Vec<Compare>),
    ExactMatch {
        pattern: String,
    },
    StringMatch {
        substring: String,
    },
    InArray {
        in_members: Vec<String>,
        not_in_members: Vec<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Compare {
    pub comparison: Comparison,
    pub n: String, // to allow all numbers
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


