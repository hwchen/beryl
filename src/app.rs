use actix_web::{
    http::Method,
    middleware,
    App,
};

use crate::backend::Backend;
use crate::handlers::{
    api_default_handler,
    api_handler,
    api_single_default_handler,
    api_single_handler,
    index_handler,
    metadata_all_handler,
    metadata_handler,
};
use crate::schema::Schema;

pub struct AppState {
    pub schema: Schema,
    pub backend: Box<Backend>,
    pub debug: bool,
}

pub fn create_app(schema: Schema, backend: Box<Backend>, debug: bool) -> App<AppState> {
    let app = App::with_state(AppState { schema, backend, debug })
        .middleware(middleware::Logger::default())
        .resource("/", |r| {
            r.method(Method::GET).with(index_handler)
        })
        .resource("/metadata", |r| {
            r.method(Method::GET).with(metadata_all_handler)
        })
        .resource("/metadata/{endpoint}", |r| {
            r.method(Method::GET).with(metadata_handler)
        })
        .resource("/api/{endpoint}.{format}", |r| {
            r.method(Method::GET).with(api_handler)
        })
        .resource("/api/{endpoint}", |r| {
            r.method(Method::GET).with(api_default_handler)
        })
        .resource("/api/{endpoint}/{id}.{format}", |r| {
            r.method(Method::GET).with(api_single_handler)
        })
        .resource("/api/{endpoint}/{id}", |r| {
            r.method(Method::GET).with(api_single_default_handler)
        });

    app
}

