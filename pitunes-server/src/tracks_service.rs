use std::{
    io::{BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

use actix_multipart::Multipart;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use diesel::prelude::*;
use futures::{StreamExt, TryStreamExt};

use crate::{
    chunker::Chunker,
    external_id::ExternalId,
    graphql_schema::RequestContext,
    models::{Album, Artist, Genre, NewAlbum, NewArtist, NewGenre, NewTrack},
    prng,
    schema::{albums, artists, genres, tracks},
};

#[post("/tracks")]
async fn post_tracks(
    context: web::Data<RequestContext>,
    req: HttpRequest,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut tracks = Vec::new();
    let conn = context.pool.get().unwrap();
    // iterate over multipart stream
    while let Ok(Some(mut field)) = payload.try_next().await {
        // File::create is blocking operation, use threadpool
        let mut tf = web::block(|| tempfile::tempfile()).await?;
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            tf = web::block(move || tf.write_all(&data).map(|_| tf)).await?;
        }
        tf.seek(SeekFrom::Start(0))?;
        let duration = mp3_duration::from_file(&tf);
        tf.seek(SeekFrom::Start(0))?;
        let tf2 = tf.try_clone()?;
        if let Ok(external_id) = conn.transaction::<_, anyhow::Error, _>(|| {
            let new_track = if let Ok(tag) = id3::Tag::read_from(tf2) {
                let track_name = tag.title().map(|t| String::from(t)).unwrap_or_else(|| {
                    let content_type = field.content_disposition().unwrap();
                    let filename = content_type.get_filename().unwrap();
                    let path = Path::new(filename);
                    let file_stem = path.file_stem().unwrap();
                    String::from(file_stem.to_str().unwrap())
                });
                let track_duration = if let Ok(duration) = duration {
                    duration.as_millis() as i32
                } else {
                    tag.duration().unwrap() as i32
                };
                let track_album_id = if let Some(album_name) = tag.album() {
                    if let Ok(album) = albums::table
                        .filter(albums::name.eq(album_name))
                        .first::<Album>(&conn)
                    {
                        Some(album.id)
                    } else {
                        let new_album = NewAlbum {
                            id: prng::rand_i32(&conn)?,
                            name: String::from(album_name),
                        };
                        diesel::insert_into(albums::table)
                            .values(&new_album)
                            .execute(&conn)?;
                        Some(new_album.id)
                    }
                } else {
                    None
                };
                let track_artist_id = if let Some(artist_name) = tag.artist() {
                    if let Ok(artist) = artists::table
                        .filter(artists::name.eq(artist_name))
                        .first::<Artist>(&conn)
                    {
                        Some(artist.id)
                    } else {
                        let new_artist = NewArtist {
                            id: prng::rand_i32(&conn)?,
                            name: String::from(artist_name),
                        };
                        diesel::insert_into(artists::table)
                            .values(&new_artist)
                            .execute(&conn)?;
                        Some(new_artist.id)
                    }
                } else {
                    None
                };
                let track_genre_id = if let Some(genre_name) = tag.genre() {
                    if let Ok(genre) = genres::table
                        .filter(genres::name.eq(genre_name))
                        .first::<Genre>(&conn)
                    {
                        Some(genre.id)
                    } else {
                        let new_genre = NewGenre {
                            id: prng::rand_i32(&conn)?,
                            name: String::from(genre_name),
                        };
                        diesel::insert_into(genres::table)
                            .values(&new_genre)
                            .execute(&conn)?;
                        Some(new_genre.id)
                    }
                } else {
                    None
                };
                let track_track_number = tag.track().map(|t| t as i32);
                NewTrack {
                    id: prng::rand_i32(&conn)?,
                    name: track_name,
                    duration: track_duration,
                    album_id: track_album_id,
                    artist_id: track_artist_id,
                    genre_id: track_genre_id,
                    track_number: track_track_number,
                }
            } else {
                let content_type = field.content_disposition().unwrap();
                let filename = content_type.get_filename().unwrap();
                let path = Path::new(filename);
                let file_stem = path.file_stem().unwrap();
                let track_name = String::from(file_stem.to_str().unwrap());
                let track_duration = duration.unwrap().as_millis() as i32;
                NewTrack {
                    id: prng::rand_i32(&conn)?,
                    name: track_name,
                    duration: track_duration,
                    album_id: None,
                    artist_id: None,
                    genre_id: None,
                    track_number: None,
                }
            };
            diesel::insert_into(tracks::table)
                .values(&new_track)
                .execute(&conn)?;
            Ok(ExternalId::from(new_track.id))
        }) {
            let reader = BufReader::new(tf);
            let mut chunker = Chunker::new(reader);
            let filepath = {
                let mut filepath = context.tracks_dir.clone();
                filepath.push(&external_id.0[..]);
                filepath.set_extension("mp3");
                filepath
            };
            // File::create is blocking operation, use threadpool
            let f = web::block(|| std::fs::File::create(filepath)).await?;
            let mut writer = BufWriter::new(f);
            while let Some(chunk) = chunker.next() {
                // filesystem operations are blocking, we have to use threadpool
                writer = web::block(move || writer.write_all(&chunk).map(|_| writer)).await?;
            }
            tracks.push(req.url_for("get_track", &[&external_id.0[..]])?.to_string());
        }
    }
    Ok(HttpResponse::Created().json(tracks))
}
