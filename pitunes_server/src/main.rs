#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod chunker;
mod db;
mod graphiql;
mod graphql_schema;
mod graphql_service;
mod models;
mod schema;
mod upload_service;

use std::fs;
use std::sync::Arc;

use actix_files::Files;
use actix_web::dev::ServiceRequest;
use actix_web::{error, web, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use base64;
use clap::{self, value_t};
use graphql_schema::{create_schema, Context};
use lazy_static::lazy_static;
use openssl::rand;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

const PITUNES_TOML: &str = "pitunes.toml";

#[derive(Serialize, Deserialize)]
struct Config {
    hashed_api_key: String,
}

lazy_static! {
    static ref CONFIG: Config = fs::read_to_string(PITUNES_TOML)
        .map(|config_string| toml::from_str(&config_string[..]))
        .map(|config| match config {
            Ok(config) => config,
            Err(_) => {
                let mut buf = [0; 32];
                rand::rand_bytes(&mut buf).unwrap();
                let api_key = base64::encode(&buf);
                println!("API_KEY={}", api_key);
                let mut hasher = Sha256::new();
                hasher.input(api_key.as_bytes());
                let hashed_api_key = format!("{:x}", hasher.result());
                let config = Config { hashed_api_key };
                let config_string = toml::to_string(&config).unwrap();
                fs::write(PITUNES_TOML, &config_string[..]).unwrap();
                config
            }
        })
        .unwrap();
}

async fn validator(req: ServiceRequest, bearer: BearerAuth) -> Result<ServiceRequest, Error> {
    let mut hasher = Sha256::new();
    hasher.input(bearer.token().as_bytes());
    let hashed_token = format!("{:x}", hasher.result());
    if hashed_token == *CONFIG.hashed_api_key {
        Ok(req)
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
                .help("Port to use (defaults to 8080)")
                .takes_value(true),
        )
        .get_matches();
    let port = value_t!(matches, "port", u16).unwrap_or(8080);

    lazy_static::initialize(&CONFIG);

    // r2d2 pool
    let pool = db::establish_connection();
    let ctx = Context { pool: pool.clone() };
    let st = Arc::new(create_schema());

    // load ssl keys
    // to create a self-signed temporary cert for testing:
    // `openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'`
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let http_server = HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .data(st.clone())
            .data(ctx.clone())
            .service(graphql_service::graphiql)
            .service(upload_service::get_upload)
            .service(
                web::scope("")
                    .wrap(auth)
                    .service(graphql_service::graphql)
                    .service(upload_service::post_upload)
                    .service(Files::new("/static", "static")),
            )
    })
    // .bind_openssl(format!("0.0.0.0:{}", port), builder)?;
    .bind_openssl(format!("127.0.0.1:{}", port), builder)?;

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
