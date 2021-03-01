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
use actix_web_static_files;
use clap::{self, value_t};
use diesel::prelude::*;
use graphql_schema::{create_schema, RequestContext};
use models::User;
use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use schema::users;
use sha2::{Digest, Sha256};

use pitunes_frontend::generate;

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
    // to create a self-signed temporary cert for testing:
    // https://letsencrypt.org/docs/certificates-for-localhost/#making-and-trusting-your-own-certificates
    let mut builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    builder
        .set_private_key_file("key.pem", SslFiletype::PEM)
        .unwrap();
    builder.set_certificate_chain_file("cert.pem").unwrap();

    let http_server = HttpServer::new(move || {
        let ctx = RequestContext::new(pool.clone(), tracks_dir.clone());
        let auth = HttpAuthentication::basic(validator);
        let generated = generate();
        App::new().wrap(auth).data(st.clone()).data(ctx).service(
            web::scope("/api")
                .service(graphql_service::graphql)
                .service(tracks_service::post_tracks)
                .service(playlists_service::get_playlist)
                .service(Files::new("/tracks", tracks_dir.clone()))
                .service(
                    web::resource("/tracks/{id}.mp3")
                        .name("get_track")
                        .to(|| HttpResponse::NotFound()),
                ) // only used for resource url generation
            )
            .service(actix_web_static_files::ResourceFiles::new("/", generated).resolve_not_found_to_root(),
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
