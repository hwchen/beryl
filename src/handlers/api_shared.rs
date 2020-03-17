use failure::Error;
use indexmap::IndexMap;
use serde_derive::{Serialize, Deserialize};
use std::convert::TryFrom;

use crate::query::Query;


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiQueryOpt {
    #[serde(flatten)]
    filters: IndexMap<String,String>,

    sort: Option<String>,
    limit: Option<String>, // includes offset
}


impl TryFrom<ApiQueryOpt> for Query {
    type Error = Error;

    fn try_from(query_opt: ApiQueryOpt) -> Result<Self, Self::Error> {

        let filters = query_opt.filters;

        let sort = query_opt.sort.map(|s| s.parse()).transpose()?;
        let limit = query_opt.limit.map(|l| l.parse()).transpose()?;

        Ok(Query {
            filters,
            sort,
            limit,
        })
    }
}
