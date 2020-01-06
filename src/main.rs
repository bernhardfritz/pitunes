#[macro_use]
extern crate actix_web;
#[macro_use]
extern crate clap;

use actix_files::Files;
use actix_web::{App, HttpServer};
use r2d2_sqlite::SqliteConnectionManager;

mod album_service;
mod artist_service;
mod genre_service;
mod migration;
mod song_service;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let matches = clap::App::new("piTunes")
    .version("0.1.0")
    .about("A server that allows you to stream your personal music collection")
    .author("Bernhard Fritz <bernhard.e.fritz@gmail.com>")
    .arg(clap::Arg::with_name("port")
        .short("p")
        .long("port")
        .value_name("PORT")
        .help("Port to use (defaults to 8080)")
        .takes_value(true))
    .get_matches();
    let port = value_t!(matches, "port", u16).unwrap_or(8080);

    // r2d2 pool
    let manager = SqliteConnectionManager::file("pitunes.db");
    let pool = r2d2::Pool::new(manager).unwrap();
    migration::migrate(&pool).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // <- store db pool in app state
            .service(album_service::get_album)
            .service(album_service::get_albums)
            .service(artist_service::get_artist)
            .service(artist_service::get_artists)
            .service(genre_service::get_genre)
            .service(genre_service::get_genres)
            .service(song_service::get_song)
            .service(song_service::get_songs)
            .service(Files::new("/static", "static"))
    })
    .bind(format!("127.0.0.1:{}", port))?
    .run()
    .await
}
