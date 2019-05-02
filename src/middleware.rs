use actix_web::HttpRequest;
use actix_web::middleware::{Middleware, Started};
use actix_web::error::{ErrorUnauthorized, ParseError};
use actix_web::Result;

pub const X_BERYL_SECRET: &str = "x-beryl-secret";

pub struct VerifyApiKey {
    secret: String,
}

impl VerifyApiKey {
    pub fn new(secret: String) -> Self {
        VerifyApiKey {
            secret,
        }
    }
}

impl<S> Middleware<S> for VerifyApiKey {
    fn start(&self, req: &HttpRequest<S>) -> Result<Started> {
        let qp_secret_is_valid = {
            let qry = req.query();
            let qp_secret = qry.get(X_BERYL_SECRET);

            qp_secret.map(|val| val == &self.secret)
                .unwrap_or(false)
        };

        let header_secret = req.headers().get(X_BERYL_SECRET);
        let header_secret_is_valid = header_secret.map(|result_val| {
            result_val.to_str().map(|val| val == self.secret).unwrap_or(false)
        }).unwrap_or(false);

        if qp_secret_is_valid || header_secret_is_valid {
            Ok(Started::Done)
        } else {
            Err(ErrorUnauthorized(ParseError::Header))
        }
    }
}
