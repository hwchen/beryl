use actix_web::{
    AsyncResponder,
    FutureResponse,
    HttpRequest,
    HttpResponse,
    Path,
};
use failure::Error;
use futures::future::{self, Future};
use indexmap::IndexMap;
use lazy_static::lazy_static;
use log::*;
use serde_derive::{Serialize, Deserialize};
use serde_qs as qs;
use std::convert::{TryFrom, TryInto};

use crate::app::AppState;
use crate::error::ServerError;
use crate::format::{FormatType, format_records};
use crate::query::Query;

/// Handles default aggregation when a format is not specified.
/// Default format is CSV.
pub fn api_default_handler(
    (req, endpoint): (HttpRequest<AppState>, Path<String>)
    ) -> FutureResponse<HttpResponse>
{
    let endpoint_format = (endpoint.into_inner(), "csv".to_owned());
    do_api(req, endpoint_format)
}

/// Handles aggregation when a format is specified.
pub fn api_handler(
    (req, endpoint_format): (HttpRequest<AppState>, Path<(String, String)>)
    ) -> FutureResponse<HttpResponse>
{
    do_api(req, endpoint_format.into_inner())
}

/// Performs data aggregation.
pub fn do_api(
    req: HttpRequest<AppState>,
    endpoint_format: (String, String),
    ) -> FutureResponse<HttpResponse>
{
    let (endpoint, format) = endpoint_format;

    let format = format.parse::<FormatType>();
    let format = match format {
        Ok(f) => f,
        Err(err) => {
            return Box::new(
                future::result(
                    Ok(HttpResponse::NotFound().json(err.to_string()))
                )
            );
        },
    };

    info!("endpoint: {}, format: {:?}", endpoint, format);

    let query = req.query_string();
    lazy_static!{
        static ref QS_NON_STRICT: qs::Config = qs::Config::new(5, false);
    }
    let api_query_res = QS_NON_STRICT.deserialize_str::<ApiQueryOpt>(&query);
    let api_query = match api_query_res {
        Ok(q) => q,
        Err(err) => {
            return Box::new(
                future::result(
                    Ok(HttpResponse::NotFound().json(err.to_string()))
                )
            );
        },
    };
    info!("query opts:{:?}", api_query);

    // Turn ApiQueryOpt into Query
    let query: Result<Query, _> = api_query.try_into();
    let query = match query {
        Ok(q) => q,
        Err(err) => {
            return Box::new(
                future::result(
                    Ok(HttpResponse::NotFound().json(err.to_string()))
                )
            );
        },
    };

    // Turn Query into QueryIr and headers (Vec<String>)
    let query_ir_headers = req
        .state()
        .schema
        .gen_query_ir(&endpoint, &query);

    let (query_ir, headers) = match query_ir_headers {
        Ok(x) => x,
        Err(err) => {
            return Box::new(
                future::result(
                    Ok(HttpResponse::NotFound().json(err.to_string()))
                )
            );
        },
    };

    let sql = req.state()
        .backend
        .generate_sql(query_ir);

    info!("Sql query: {}", sql);
    info!("Headers: {:?}", headers);

    // Now pass request to backend
    req.state()
        .backend
        .exec_sql(sql)
        .and_then(move |df| {
            match format_records(&headers, df, format) {
                Ok(res) => Ok(HttpResponse::Ok().body(res)),
                Err(err) => Ok(HttpResponse::NotFound().json(err.to_string())),
            }
        })
        .map_err(move |e| {
            if req.state().debug {
                ServerError::Db { cause: e.to_string() }.into()
            } else {
                ServerError::Db { cause: "Internal Server Error 1010".to_owned() }.into()
            }
        })
        .responder()
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiQueryOpt {
    #[serde(flatten)]
    interface: IndexMap<String,String>,

    sort: Option<String>,
    limit: Option<String>, // includes offset
}

impl TryFrom<ApiQueryOpt> for Query {
    type Error = Error;

    fn try_from(query_opt: ApiQueryOpt) -> Result<Self, Self::Error> {
        Ok(Query {})
    }
}
