#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod chunker;
mod db;
mod graphql_schema;
mod graphql_service;
mod models;
mod playlists_service;
mod schema;
mod tracks_service;
mod uuid;

use std::sync::Arc;

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{
    dev::ServiceRequest,
    error,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use clap::{self, value_t};
use diesel::prelude::*;
use graphql_schema::{create_schema, RequestContext};
use models::User;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use schema::users;
use sha2::{Digest, Sha256};

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    if let Some(context) = req.app_data::<Data<RequestContext>>() {
        let conn = context.pool.get().unwrap();
        if let Ok(user) = users::table
            .filter(users::username.eq(credentials.user_id()))
            .first::<User>(&conn)
        {
            if let Some(user_password) = user.password {
                if let Some(credentials_password) = credentials.password() {
                    let mut hasher = Sha256::new();
                    hasher.input(credentials_password.as_bytes());
                    let hashed_credentials_password = format!("{:x}", hasher.result());
                    if user_password == hashed_credentials_password {
                        Ok(req)
                    } else {
                        Err(error::ErrorUnauthorized(""))
                    }
                } else {
                    Err(error::ErrorUnauthorized(""))
                }
            } else {
                if credentials.password().is_none() {
                    Ok(req)
                } else {
                    Err(error::ErrorUnauthorized(""))
                }
            }
        } else {
            Err(error::ErrorUnauthorized(""))
        }
    } else {
        Err(error::ErrorUnauthorized(""))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("piTunes")
        .version("0.1.0")
        .about("A server that allows you to stream your personal music collection")
        .author("Bernhard Fritz <bernhard.e.fritz@gmail.com>")
        .arg(
            clap::Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Port to use (defaults to 8443)")
                .takes_value(true),
        )
        .get_matches();
    let port = value_t!(matches, "port", u16).unwrap_or(8443);

    // r2d2 pool
    let pool = db::establish_connection();
    let st = Arc::new(create_schema());

    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // https://letsencrypt.org/docs/certificates-for-localhost/#making-and-trusting-your-own-certificates
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let http_server = HttpServer::new(move || {
        let ctx = RequestContext::new(pool.clone());
        let cors = Cors::new()
            // .allowed_origin("http://localhost:3000")
            // .allowed_methods(vec!["GET", "POST"])
            // .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            // .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            // .max_age(3600)
            .finish();
        let auth = HttpAuthentication::basic(validator);
        App::new()
            .wrap(auth)
            .wrap(cors)
            .data(st.clone())
            .data(ctx)
            .service(
                web::scope("/api")
                    .service(graphql_service::graphql)
                    .service(tracks_service::post_tracks)
                    .service(playlists_service::get_playlist)
                    .service(Files::new("/tracks", "tracks"))
                    .service(
                        web::resource("/tracks/{uuid}.mp3")
                            .name("get_track")
                            .to(|| HttpResponse::NotFound()),
                    ), // only used for resource url generation
            )
    })
    .bind_openssl(format!("0.0.0.0:{}", port), builder)?;

    println!(
        r#"       _ _____
 _ __ (_)_   _|   _ _ __   ___  ___
| '_ \| | | || | | | '_ \ / _ \/ __|
| |_) | | | || |_| | | | |  __/\__ \
| .__/|_| |_| \__,_|_| |_|\___||___/ v{}
|_|
"#,
        env!("CARGO_PKG_VERSION")
    );

    for (addrs, scheme) in http_server.addrs_with_scheme() {
        println!("Listening on {}://{}", scheme, addrs);
    }

    http_server.run().await
}
