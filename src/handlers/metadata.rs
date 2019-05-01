use actix_web::{
    HttpRequest,
    HttpResponse,
    Path,
    Result as ActixResult,
};
use failure::format_err;
use serde_derive::Serialize;

use crate::app::AppState;
use crate::schema::{Endpoint, FilterType};

pub fn metadata_handler(req: HttpRequest<AppState>, endpoint_path: Path<String>) -> ActixResult<HttpResponse> {
    let endpoint = req.state().schema.get_endpoint(&endpoint_path)
        .ok_or_else(|| format_err!("Endpoint '{}' for metadata not found", endpoint_path))?;

    let metadata = endpoint_metadata(&endpoint);

    Ok(HttpResponse::Ok().json(metadata))
}

pub fn metadata_all_handler(req: HttpRequest<AppState>) -> ActixResult<HttpResponse> {
    let metadatas: Vec<_> = req.state().schema.endpoints.iter().map(|e| endpoint_metadata(&e)).collect();

    Ok(HttpResponse::Ok().json(metadatas))
}

#[derive(Debug, Serialize)]
struct Metadata {
    //annotations:
    name: String,
    fields: Vec<FieldMetadata>,
    filters: Vec<FilterMetadata>,
    primary_field: Option<String>,
}

#[derive(Debug, Serialize)]
struct FieldMetadata {
    //annotations
    name: String,
}

#[derive(Debug, Serialize)]
struct FilterMetadata {
    //annotations
    name: String,
    filter_type: FilterType,
}

fn endpoint_metadata(endpoint: &Endpoint) -> Metadata {
    let fields = endpoint.interface.0.iter()
        .filter(|(_, v)| v.visible)
        .map(|(k, _)| {
            FieldMetadata {
                name: k.to_owned(),
            }
        })
        .collect();

    let filters = endpoint.interface.0.iter()
        .map(|(k, v)| {
            FilterMetadata {
                name: k.to_owned(),
                filter_type: v.filter_type.clone(),
            }
        })
        .collect();

    Metadata {
        name: endpoint.name.clone(),
        fields,
        filters,
        primary_field: endpoint.primary.clone(),
    }
}
