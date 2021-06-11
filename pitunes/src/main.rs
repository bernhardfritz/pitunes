#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

mod chunker;
mod db;
mod external_id;
mod graphql_schema;
mod graphql_service;
mod mk_certs;
mod models;
mod playlists_service;
mod prng;
mod schema;
mod tracks_service;

use std::sync::Arc;

use actix_files::Files;
use actix_web::{
    dev::ServiceRequest,
    error,
    web::{self, Data},
    App, Error, HttpResponse, HttpServer,
};
use actix_web_httpauth::{extractors::basic::BasicAuth, middleware::HttpAuthentication};
use actix_web_middleware_redirect_scheme::RedirectSchemeBuilder;
use actix_web_static_files;
use clap::{self, value_t};
use diesel::prelude::*;
use graphql_schema::{create_schema, RequestContext};
use mk_certs::{mk_ca_cert, mk_ca_signed_cert};
use models::User;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use schema::users;
use sha2::{Digest, Sha256};

async fn validator(req: ServiceRequest, credentials: BasicAuth) -> Result<ServiceRequest, Error> {
    let valid = {
        let mut valid = false;
        if let Some(context) = req.app_data::<Data<RequestContext>>() {
            let conn = context.pool.get().unwrap();
            if let Ok(user) = users::table
                .find(credentials.user_id())
                .get_result::<User>(&conn)
            {
                let hash = Sha256::digest(
                    credentials
                        .password()
                        .map(|password| password.as_bytes())
                        .unwrap_or_default(),
                );
                valid = user.password == hash.as_slice();
            }
        }
        valid
    };
    if valid {
        Ok(req)
    } else {
        Err(error::ErrorUnauthorized(""))
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("pitunes")
        .version("0.1.0")
        .about("A Raspberry Pi compatible tool to manage and stream your personal music collection remotely.")
        .author("Bernhard Fritz <bernhard.e.fritz@gmail.com>")
        .arg(
            clap::Arg::with_name("http-port")
                .short("p")
                .long("http-port")
                .value_name("HTTP PORT")
                .help("HTTP port to use (defaults to 8080)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("https-port")
                .short("s")
                .long("https-port")
                .value_name("HTTPS PORT")
                .help("HTTPS port to use (defaults to 8443)")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("cert")
                .short("c")
                .long("cert")
                .value_name("FILE")
                .help("Certificate to use (defaults to self-signed)")
        )
        .arg(
            clap::Arg::with_name("key")
                .short("k")
                .long("key")
                .value_name("FILE")
                .help("Private key to use (defaults to self-signed)")
        )
        .get_matches();
    let http_port = value_t!(matches, "http-port", u16).unwrap_or(8080);
    let https_port = value_t!(matches, "https-port", u16).unwrap_or(8443);
    let cert = value_t!(matches, "cert", String);
    let key = value_t!(matches, "key", String);

    let config_dir = {
        let mut config_dir = dirs::config_dir().unwrap();
        config_dir.push("pitunes");
        config_dir
    };

    let tracks_dir = {
        let mut tracks_dir = config_dir.clone();
        tracks_dir.push("tracks");
        std::fs::create_dir_all(tracks_dir.as_path())?;
        tracks_dir
    };

    // r2d2 pool
    let pool = {
        let pitunes_db = {
            let mut pitunes_db = config_dir.clone();
            pitunes_db.push("pitunes");
            pitunes_db.set_extension("db");
            pitunes_db.into_os_string().into_string().unwrap()
        };
        db::establish_connection(&pitunes_db[..])
    };
    let st = Arc::new(create_schema());

    // load ssl keys
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    if let (Ok(cert), Ok(key)) = (cert, key) {
        builder.set_private_key_file(key, SslFiletype::PEM).unwrap();
        builder.set_certificate_chain_file(cert).unwrap();
    } else {
        let (ca_cert, ca_privkey) = mk_ca_cert().unwrap();
        let (cert, _privkey) = mk_ca_signed_cert(&ca_cert, &ca_privkey).unwrap();
        builder.set_private_key(&_privkey).unwrap();
        builder.set_certificate(&cert).unwrap();
    }

    let http_server = HttpServer::new(move || {
        let ctx = RequestContext::new(pool.clone(), tracks_dir.clone());
        let auth = HttpAuthentication::basic(validator);
        let pitunes_frontend = pitunes_frontend::generate();
        App::new()
            .wrap(auth)
            .wrap(RedirectSchemeBuilder::new().replacements(&[(format!(":{}", http_port), format!(":{}", https_port))]).build())
            .data(st.clone())
            .data(ctx)
            .service(
                web::scope("/api")
                    .service(graphql_service::graphql)
                    .service(tracks_service::post_tracks)
                    .service(playlists_service::get_playlist)
                    .service(Files::new("/tracks", tracks_dir.clone()))
                    .service(
                        web::resource("/tracks/{id}.mp3")
                            .name("get_track")
                            .to(|| HttpResponse::NotFound()),
                    ), // only used for resource url generation
            )
            .service(
                actix_web_static_files::ResourceFiles::new("/", pitunes_frontend)
                    .resolve_not_found_to_root(),
            )
    })
    .bind(format!("0.0.0.0:{}", http_port))?
    .bind_openssl(format!("0.0.0.0:{}", https_port), builder)?;

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
