use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Artist {
    guid: String,
    created_at: DateTime<Utc>,
    name: String,
}

#[get("/artists/{guid}")]
async fn get_artist(
    path: web::Path<String>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let guid = path.into_inner();
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT artist.guid, artist.created_at, artist.name FROM artist
            WHERE artist.guid=$1";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_row(params![guid], |row| {
            let artist = Artist {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
            };
            Ok(artist)
        })
    })
    .await
    .map(|artist| HttpResponse::Ok().json(artist))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

#[get("/artists")]
async fn get_artists(
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT artist.guid, artist.created_at, artist.name FROM artist";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map(params![], |row| {
            let artist = Artist {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
            };
            Ok(artist)
        })
        .and_then(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Artist>>()))
    })
    .await
    .map(|artists| HttpResponse::Ok().json(artists))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}
