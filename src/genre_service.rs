use actix_web::{web, Error, HttpResponse};
use chrono::{DateTime, Utc};
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use serde::Serialize;

#[derive(Debug, Serialize)]
struct Genre {
    guid: String,
    created_at: DateTime<Utc>,
    name: String,
}

#[get("/genres/{guid}")]
async fn get_genre(
    path: web::Path<String>,
    pool: web::Data<Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, Error> {
    let guid = path.into_inner();
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT genre.guid, genre.created_at, genre.name FROM genre
            WHERE genre.guid=?1";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_row(params![guid], |row| {
            let genre = Genre {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
            };
            Ok(genre)
        })
    })
    .await
    .map(|genre| HttpResponse::Ok().json(genre))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}

#[get("/genres")]
async fn get_genres(pool: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, Error> {
    // execute sync code in threadpool
    let res = web::block(move || {
        let conn = pool.get().unwrap();
        let sql = "SELECT genre.guid, genre.created_at, genre.name FROM genre";
        let mut stmt = conn.prepare(sql).unwrap();
        stmt.query_map(params![], |row| {
            let genre = Genre {
                guid: row.get(0)?,
                created_at: row.get(1)?,
                name: row.get(2)?,
            };
            Ok(genre)
        })
        .and_then(|mapped_rows| Ok(mapped_rows.map(|row| row.unwrap()).collect::<Vec<Genre>>()))
    })
    .await
    .map(|genres| HttpResponse::Ok().json(genres))
    .map_err(|_| HttpResponse::InternalServerError())?;
    Ok(res)
}
