use actix_web::{Error, HttpResponse, http::StatusCode};
use askama::Template;

use crate::args::{Args, ARGS};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub status_code: StatusCode,
    pub args: &'a Args,
}

pub async fn not_found() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::NotFound()
        .content_type("text/html")
        .body(ErrorTemplate { 
            status_code: StatusCode::NOT_FOUND, 
            args: &ARGS 
        }.render().unwrap()))
}
