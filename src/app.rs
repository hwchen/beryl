use actix_web::{
    http::Method,
    middleware,
    App,
};

use crate::backend::Backend;
use crate::handlers::{
    api_default_handler,
    api_handler,
    index_handler,
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
        .resource("/api/{cube}", |r| {
            r.method(Method::GET).with(api_default_handler)
        })
        .resource("/api/{cube}.{format}", |r| {
            r.method(Method::GET).with(api_handler)
        });

    app
}

