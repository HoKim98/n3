use rocket::http::Method;
use rocket_cors::{AllowedHeaders, Cors, CorsOptions};

use crate::global::allowed_origins;

pub fn fairing() -> Cors {
    CorsOptions {
        allow_credentials: true,
        allowed_origins: allowed_origins(),
        allowed_methods: vec![
            Method::Get,
            Method::Post,
            Method::Put,
            Method::Delete,
            Method::Options,
        ]
        .into_iter()
        .map(From::from)
        .collect(),
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Content-Type",
            "Origin",
            "X-Amz-Date",
            "X-Amz-Security-Token",
            "X-Api-Key",
        ]),
        ..Default::default()
    }
    .to_cors()
    .expect("error creating CORS fairing")
}
