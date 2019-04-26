use actix_web::{
    HttpRequest,
    HttpResponse,
    Result as ActixResult,
};
use serde_derive::Serialize;
use structopt::clap::crate_version;

use crate::app::AppState;

pub fn index_handler(_req: HttpRequest<AppState>) -> ActixResult<HttpResponse> {
    Ok(HttpResponse::Ok().json(
        Status {
            status: "ok".into(),
            beryl_version: crate_version!().to_owned(),
        }
    ))
}

#[derive(Debug, Serialize)]
struct Status {
    status: String,
    beryl_version: String,
}

