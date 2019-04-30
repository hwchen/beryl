use failure::{Error, format_err, bail};
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
                match &filter_query.split(".").collect::<Vec<_>>()[..] {
                    [constraint_type, members] => {
                        let comparison = constraint_type.parse::<Comparison>()?;
                        let n = members.parse::<i64>()?;

                        Constraint::Compare {
                            comparison,
                            n,
                        }
                    },
                    _ => bail!("Could not parse a Comparison for filter {}", name),
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
    Compare {
        comparison: Comparison,
        n: i64,
    },
    StringMatch {
        substring: String,
    },
    InArray {
        in_members: Vec<String>,
        not_in_members: Vec<String>,
    },
}

impl FromStr for Constraint {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s.split(".").collect::<Vec<_>>()[..] {
            [constraint_type, members] => {
                match *constraint_type {
                    "match" => {
                        Ok(Constraint::StringMatch {
                            substring: members.to_string(),
                        })
                    },
                    "in_array" => {
                        let mut in_members = vec![];
                        let mut not_in_members = vec![];

                        for member in members.split(",") {
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

                        Ok(Constraint::InArray {
                            in_members,
                            not_in_members,
                        })
                    },
                    _ => {
                        let comparison = constraint_type.parse::<Comparison>()?;
                        let n = members.parse::<i64>()?;

                        Ok(Constraint::Compare {
                            comparison,
                            n,
                        })
                    },
                }
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

