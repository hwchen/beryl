use std::collections::HashMap;

use actix_web::{
    AsyncResponder,
    FutureResponse,
    HttpRequest,
    HttpResponse,
    Path,
};
use failure::Error;
use futures::future::{self, *};
use lazy_static::lazy_static;
use log::*;
use serde_qs as qs;
use std::convert::TryInto;

use crate::app::AppState;
use crate::dataframe::{DataFrame, Column, ColumnData};
use crate::error::ServerError;
use crate::format::{FormatType, format_records};
use crate::query::Query;
use super::api_shared::ApiQueryOpt;
use super::util;


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

    let sql_queries = req.state()
        .backend
        .generate_sql(query_ir);

    info!("Headers: {:?}", headers);

    // Joins all the futures for each TsQuery
    let futs: JoinAll<Vec<Box<dyn Future<Item=DataFrame, Error=Error>>>> = join_all(sql_queries
        .iter()
        .map(|sql| {
            info!("Sql query: {}", sql);

            req.state()
                .backend
                .exec_sql(sql.clone())
        })
        .collect()
    );

    // Process data received once all futures are resolved and return response
    futs
        .and_then(move |dfs| {
            let table_count = match dfs.get(0) {
                Some(df) => get_count(df),
                None => return Ok(HttpResponse::NotFound().json("Unable to get table count.".to_string()))
            };

            let filter_count = match dfs.get(1) {
                Some(df) => get_count(df),
                None => return Ok(HttpResponse::NotFound().json("Unable to get filter count.".to_string()))
            };

            let mut metadata = HashMap::new();

            metadata.insert("table_count".to_string(), table_count);
            metadata.insert("filter_count".to_string(), filter_count);

            let df = match dfs.get(2) {
                Some(df) => df.clone(),
                None => return Ok(HttpResponse::NotFound().json("Unable to get data.".to_string()))
            };

            let content_type = util::format_to_content_type(&format);

            match format_records(&headers, df, format, metadata) {
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


fn get_count(df: &DataFrame) -> u64 {
    match &df.columns[0].column_data {
        ColumnData::UInt64(data)=> {
            data[0]
        },
        _ => {
            // Should never get here as counts will always return UInt64
            0
        }
    }
}
