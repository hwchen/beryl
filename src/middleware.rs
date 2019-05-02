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
        let secret_header = req.headers().get(X_BERYL_SECRET);

        let qp_secret_is_valid = {
            let qry = req.query();
            let secret_query_param = qry.get(X_BERYL_SECRET);

            match secret_query_param {
                Some(val) => val == &self.secret,
                _ => false
            }
        };

        let header_secret_is_valid = match secret_header {
            Some(result_val) => match result_val.to_str() {
                    Ok(val) => val == self.secret,
                    _ => false
            },
            _ => false
        };

        if qp_secret_is_valid || header_secret_is_valid {
            Ok(Started::Done)
        } else {
            Err(ErrorUnauthorized(ParseError::Header))
        }
    }
}
