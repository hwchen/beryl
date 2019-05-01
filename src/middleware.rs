use actix_web::HttpRequest;
use actix_web::middleware::{Middleware, Started};
use actix_web::error::{ErrorUnauthorized, ParseError};
use actix_web::Result;

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
        let secret = req.headers()
            .get("x-beryl-secret")
            .ok_or(ErrorUnauthorized(ParseError::Header))?
            .to_str()
            .map_err(ErrorUnauthorized)?;

        if secret == self.secret {
            Ok(Started::Done)
        } else {
            Err(ErrorUnauthorized(ParseError::Header))
        }
    }
}
