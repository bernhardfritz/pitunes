use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Song {
    guid: String,
    created_at: DateTime<Utc>,
    name: String,
    duration: u32,
    artist: Option<String>,
    album: Option<String>,
    genre: Option<String>,
    track_number: Option<u32>,
}

#[get("/songs/{guid}")]
async fn get_song(
    path: web::Path<String>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let guid = path.into_inner();
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT song.guid, song.created_at, song.name, song.duration, artist.name, album.name, genre.name, song.track_number FROM song
            LEFT JOIN artist ON song.artist_id = artist.id
            LEFT JOIN album ON song.album_id = album.id
            LEFT JOIN genre ON song.genre_id = genre.id
            WHERE song.guid=$1";
        conn.query_row(sql, params![guid], |row| {
            let song = Song {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
                duration: row.get(3)?,
                artist: row.get(4)?,
                album: row.get(5)?,
                genre: row.get(6)?,
                track_number: row.get(7)?,
            };
            Ok(song)
        })
    })
    .await
    .map(|song| HttpResponse::Ok().json(song))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

#[get("/songs")]
async fn get_songs(pool: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, Error> {
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT song.guid, song.created_at, song.name, song.duration, artist.name, album.name, genre.name, song.track_number FROM song
            LEFT JOIN artist ON song.artist_id = artist.id
            LEFT JOIN album ON song.album_id = album.id
            LEFT JOIN genre ON song.genre_id = genre.id";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map(params![], |row| {
            let song = Song {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
                duration: row.get(3)?,
                artist: row.get(4)?,
                album: row.get(5)?,
                genre: row.get(6)?,
                track_number: row.get(7)?,
            };
            Ok(song)
        })
        .and_then(|mapped_rows| {
            Ok(mapped_rows
                .map(|row| row.unwrap())
                .collect::<Vec<Song>>())
        })
    })
    .await
    .map(|songs| HttpResponse::Ok().json(songs))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}
