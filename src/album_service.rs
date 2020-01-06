use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Album {
    guid: String,
    created_at: DateTime<Utc>,
    name: String,
    artist: String,
}

#[get("/albums/{guid}")]
async fn get_album(
    path: web::Path<String>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let guid = path.into_inner();
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT album.guid, album.created_at, album.name, artist.name FROM album
            LEFT JOIN artist ON album.artist_id = artist.id
            WHERE album.guid=$1";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_row(params![guid], |row| {
            let album = Album {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
                artist: row.get(3)?,
            };
            Ok(album)
        })
    })
    .await
    .map(|album| HttpResponse::Ok().json(album))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

#[get("/albums")]
async fn get_albums(pool: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, Error> {
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT album.guid, album.created_at, album.name, artist.name FROM album
            LEFT JOIN artist ON album.artist_id = artist.id";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map(params![], |row| {
            let album = Album {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
                artist: row.get(3)?,
            };
            Ok(album)
        })
        .and_then(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Album>>()))
    })
    .await
    .map(|albums| HttpResponse::Ok().json(albums))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}
