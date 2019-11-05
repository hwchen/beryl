use actix_web::{
    AsyncResponder,
    FutureResponse,
    HttpRequest,
    HttpResponse,
    Path,
};
use futures::future::{self, Future};
use indexmap::IndexMap;
use log::*;

use crate::app::AppState;
use crate::error::ServerError;
use crate::format::{FormatType, format_records};
use crate::query::Query;
use super::util;


/// Handles default aggregation when a format is not specified.
/// Default format is CSV.
pub fn api_single_default_handler(
    (req, endpoint_id): (HttpRequest<AppState>, Path<(String, String)>)
    ) -> FutureResponse<HttpResponse>
{
    let endpoint_id = endpoint_id.into_inner();
    let endpoint_id_format = (endpoint_id.0, endpoint_id.1, "csv".to_owned());
    do_api_single(req, endpoint_id_format)
}


/// Handles aggregation when a format is specified.
pub fn api_single_handler(
    (req, endpoint_id_format): (HttpRequest<AppState>, Path<(String, String, String)>)
    ) -> FutureResponse<HttpResponse>
{
    do_api_single(req, endpoint_id_format.into_inner())
}


/// Performs data aggregation.
pub fn do_api_single(
    req: HttpRequest<AppState>,
    endpoint_id_format: (String, String, String),
    ) -> FutureResponse<HttpResponse>
{
    let (endpoint, id, format) = endpoint_id_format;

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

    // populate filter with just query where primary = id
    let primary = req.state()
        .schema
        .endpoints
        .iter()
        .find(|e| e.name == endpoint)
        .map(|e| e.primary.clone());


    // unwrap primary wrappers
    let primary = match primary {
        Some(s) => s,
        None => {
            return Box::new(future::result(
                Ok(HttpResponse::NotFound().json("No primary key for the requested endpoint")
            )));
        },
    };


    let primary = match primary {
        Some(s) => s,
        None => {
            return Box::new(future::result(
                Ok(HttpResponse::NotFound().json("No primary key defined on requested endpoint")
            )));
        },
    };
    info!("Primary: {}", primary);

    let id = match id.parse::<i64>() {
        Ok(i) => i,
        Err(_) => {
            return Box::new(future::result(
                Ok(HttpResponse::NotFound().json("Currently primary keys must be u64")
            )));
        },
    };

    let mut filters = IndexMap::new();
    filters.insert(
        primary,
        format!("eq.{}", id),
    );

    let query = Query {
        filters,
        sort: None,
        limit: None,
    };

    // Turn Query into QueryIr and headers (Vec<String>)
    let query_ir_headers = req
        .state()
        .schema
        .gen_query_ir(&endpoint, &query, &req.state().sql_templates);

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
            let content_type = util::format_to_content_type(&format);

            match format_records(&headers, df, format) {
                Ok(res) => Ok(HttpResponse::Ok()
                    .set(content_type)
                    .body(res)),
                Err(err) => Ok(HttpResponse::NotFound().json(err.to_string())),
            }
        })
        .map_err(move |e| {
            error!("{}, {}", e.to_string(), e.as_fail());

            if req.state().debug {
                ServerError::Db { cause: e.to_string() }.into()
            } else {
                ServerError::Db { cause: "Internal Server Error 1010".to_owned() }.into()
            }
        })
        .responder()
}

