use actix_web::dev::JsonBody::Body;
use actix_web::error::UrlencodedError::ContentType;
use actix_web::web::Path;
use actix_web::{get, web, HttpResponse};
use askama::Template;
use std::io::ErrorKind::NotFound;
use std::marker::PhantomData;

#[get("/static/{resource}")]
pub async fn static_resources(resource_id: web::Path<String>) -> HttpResponse {
    return match resource_id.into_inner().as_str() {
        "water.css" => HttpResponse::Ok()
            .content_type("text/css")
            .body(include_bytes!("../../templates/static/water.css").to_vec()),

        "icon.ico" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(include_bytes!("../../templates/static/icon.ico").to_vec()),
        "icon-16x16.png" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(include_bytes!("../../templates/static/icon-16x16.png").to_vec()),
        "icon-32x32.png" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(include_bytes!("../../templates/static/icon-32x32.png").to_vec()),
        "icon-192x192.png" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(include_bytes!("../../templates/static/icon-192x192.png").to_vec()),
        "icon-512x512.png" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(include_bytes!("../../templates/static/icon-512x512.png").to_vec()),
        "apple-touch-icon.png" => HttpResponse::Ok()
            .content_type("image/x-icon")
            .body(include_bytes!("../../templates/static/apple-touch-icon.png").to_vec()),

        _ => HttpResponse::NotFound().content_type("text/html").finish(),
    };
}
