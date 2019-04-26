use actix_web::{
    AsyncResponder,
    FutureResponse,
    HttpRequest,
    HttpResponse,
    Path,
};
use failure::Error;
use futures::future::{self, Future};
use lazy_static::lazy_static;
use log::*;
use serde_derive::{Serialize, Deserialize};
use serde_qs as qs;
use std::convert::{TryFrom, TryInto};

use crate::app::AppState;

/// Handles default aggregation when a format is not specified.
/// Default format is CSV.
pub fn api_default_handler(
    (req, cube): (HttpRequest<AppState>, Path<String>)
    ) -> FutureResponse<HttpResponse>
{
    let cube_format = (cube.into_inner(), "csv".to_owned());
    do_api(req, cube_format)
}

/// Handles aggregation when a format is specified.
pub fn api_handler(
    (req, cube_format): (HttpRequest<AppState>, Path<(String, String)>)
    ) -> FutureResponse<HttpResponse>
{
    do_api(req, cube_format.into_inner())
}

/// Performs data aggregation.
pub fn do_api(
    req: HttpRequest<AppState>,
    cube_format: (String, String),
    ) -> FutureResponse<HttpResponse>
{
    let (cube, format) = cube_format;

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

    info!("cube: {}, format: {:?}", cube, format);

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

//    req.state()
//        .backend
//        .exec_sql(sql)
//        .and_then(move |df| {
//            match format_records(&headers, df, format) {
//                Ok(res) => Ok(HttpResponse::Ok().body(res)),
//                Err(err) => Ok(HttpResponse::NotFound().json(err.to_string())),
//            }
//        })
//        .map_err(move |e| {
//            if req.state().debug {
//                ServerError::Db { cause: e.to_string() }.into()
//            } else {
//                ServerError::Db { cause: "Internal Server Error 1010".to_owned() }.into()
//            }
//        })
//        .responder()
    Box::new(future::ok(HttpResponse::Ok().body("test")))
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ApiQueryOpt {
    measures: Option<Vec<String>>,
    filters: Option<Vec<String>>,
    sort: Option<String>,
    limit: Option<String>,
}

// Formatting: move to other mod

use failure::format_err;

#[derive(Debug, Clone)]
enum FormatType {
    Csv,
    JsonRecords,
    JsonArrays,
}

impl std::str::FromStr for FormatType {
    type Err = Error;

    fn  from_str(s: &str) -> Result< Self, Self::Err> {
        match s {
            "csv" =>         Ok(FormatType::Csv),
            "jsonrecords" => Ok(FormatType::JsonRecords),
            "jsonarrays" =>  Ok(FormatType::JsonArrays),
            _ => Err(format_err!("Could not parse format type")),
        }
    }
}

