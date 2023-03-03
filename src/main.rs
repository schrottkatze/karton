extern crate core;

use crate::args::ARGS;
use crate::endpoints::{
    create, edit, errors, info, pasta as pasta_endpoint, pastalist, qr, remove, static_resources,
};
use crate::pasta::Pasta;
use crate::util::dbio;
use actix_web::middleware::Condition;
use actix_web::{middleware, web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use chrono::Local;
use env_logger::Builder;
use futures::lock::Mutex;
use log::LevelFilter;
use std::fs;
use std::io::Write;

pub mod args;
pub mod pasta;

pub mod util {
    pub mod pasta_id_converter;
    pub mod auth;
    pub mod dbio;
    pub mod hashids;
    pub mod misc;
    pub mod syntaxhighlighter;
}

pub mod endpoints {
    pub mod create;
    pub mod edit;
    pub mod errors;
    pub mod info;
    pub mod pasta;
    pub mod pastalist;
    pub mod qr;
    pub mod remove;
    pub mod static_resources;
}

pub struct AppState {
    pub pastas: Mutex<Vec<Pasta>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Info)
        .init();

    log::info!(
        "MicroBin starting on http://{}:{}",
        ARGS.bind.to_string(),
        ARGS.port.to_string()
    );

    match fs::create_dir_all("./pasta_data/public") {
        Ok(dir) => dir,
        Err(error) => {
            log::error!("Couldn't create data directory ./pasta_data/public/: {error:?}");
            panic!("Couldn't create data directory ./pasta_data/public/: {error:?}");
        }
    };

    let data = web::Data::new(AppState {
        pastas: Mutex::new(dbio::load_from_file().unwrap()),
    });

    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .wrap(middleware::NormalizePath::trim())
            .service(create::index)
            .service(info::info)
            .route(
                &format!("/{}/{{id}}", ARGS.pasta_endpoint), 
                web::get().to(pasta_endpoint::getpasta)
            )
            .route(
                &format!("/{}/{{id}}", ARGS.raw_endpoint), 
                web::get().to(pasta_endpoint::getrawpasta)
            )
            .route(
                &format!("/{}/{{id}}", ARGS.url_endpoint), 
                web::get().to(pasta_endpoint::redirecturl)
            )
            //.service(pasta_endpoint::getpasta)
            //.service(pasta_endpoint::getrawpasta)
            //.service(pasta_endpoint::redirecturl)
            .service(edit::get_edit)
            .service(edit::post_edit)
            .service(static_resources::static_resources)
            .service(qr::getqr)
            .service(actix_files::Files::new("/file", "./pasta_data/public/"))
            .service(web::resource("/upload").route(web::post().to(create::create)))
            .default_service(web::route().to(errors::not_found))
            .wrap(middleware::Logger::default())
            .service(remove::remove)
            .service(pastalist::list)
            .wrap(Condition::new(
                ARGS.auth_username.is_some(),
                HttpAuthentication::basic(util::auth::auth_validator),
            ))
    })
    .bind((ARGS.bind, ARGS.port))?
    .workers(ARGS.threads as usize)
    .run()
    .await
}
