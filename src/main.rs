#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;

use crate::graphql_schema::{create_schema, Context};
use actix_files::Files;
use actix_web::{App, HttpServer};

mod db;
mod graphql_schema;
mod graphql_service;
mod models;
mod schema;
mod upload_service;

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

    // r2d2 pool
    let pool = db::establish_connection();
    let ctx = Context { pool: pool.clone() };
    let st = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(st.clone())
            .data(ctx.clone())
            .service(graphql_service::graphql)
            .service(graphql_service::graphiql)
            .service(upload_service::upload)
            .service(Files::new("/static", "static"))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
