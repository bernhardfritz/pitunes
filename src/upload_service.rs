use std::io::Write;
use std::path::Path;

use crate::graphql_schema::Context;
use crate::models::{Album, Artist, Genre, NewAlbum, NewArtist, NewGenre, NewTrack, Track};
use crate::schema::{albums, artists, genres, tracks};
use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::StreamExt;
use tempfile::NamedTempFile;

#[post("/upload")]
async fn upload(
    context: web::Data<Context>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    use diesel::result::Error;
    let mut vec = Vec::new();
    // iterate over multipart stream
    while let Some(item) = payload.next().await {
        let mut field = item?;
        // File::create is blocking operation, use threadpool
        let mut file = web::block(|| NamedTempFile::new()).await.unwrap();
        // Field in turn is stream of *Bytes* object
        while let Some(chunk) = field.next().await {
            let data = chunk.unwrap();
            // filesystem operations are blocking, we have to use threadpool
            file = web::block(move || file.write_all(&data).map(|_| file)).await?;
        }
        let f = file.reopen().unwrap();
        let tag_result = web::block(|| id3::Tag::read_from(f)).await;
        let conn = context.pool.get().unwrap();
        let track = conn
            .transaction::<_, Error, _>(|| match tag_result {
                Ok(tag) => {
                    let track_name = tag.title().map(|t| String::from(t)).unwrap_or_else(|| {
                        let content_type = field.content_disposition().unwrap();
                        let filename = content_type.get_filename().unwrap();
                        let path = Path::new(filename);
                        let file_stem = path.file_stem().unwrap();
                        String::from(file_stem.to_str().unwrap())
                    });
                    let track_duration = tag.duration();
                    let track_album_id = match tag.album() {
                        Some(album_name) => {
                            use self::albums::dsl::*;
                            let new_album = NewAlbum {
                                name: String::from(album_name),
                            };
                            let album_result =
                                albums.filter(name.eq(album_name)).first::<Album>(&conn);
                            match album_result {
                                Ok(album) => Some(album.id),
                                Err(_) => {
                                    diesel::insert_into(albums)
                                        .values(&new_album)
                                        .execute(&conn)
                                        .expect("Error saving new album");
                                    let album_result =
                                        albums.filter(name.eq(album_name)).first::<Album>(&conn);
                                    match album_result {
                                        Ok(album) => Some(album.id),
                                        Err(_) => None,
                                    }
                                }
                            }
                        }
                        None => None,
                    };
                    let track_artist_id = match tag.artist() {
                        Some(artist_name) => {
                            use self::artists::dsl::*;
                            let new_artist = NewArtist {
                                name: String::from(artist_name),
                            };
                            let artist_result =
                                artists.filter(name.eq(artist_name)).first::<Artist>(&conn);
                            match artist_result {
                                Ok(artist) => Some(artist.id),
                                Err(_) => {
                                    diesel::insert_into(artists)
                                        .values(&new_artist)
                                        .execute(&conn)
                                        .expect("Error saving new artist");
                                    let artist_result =
                                        artists.filter(name.eq(artist_name)).first::<Artist>(&conn);
                                    match artist_result {
                                        Ok(artist) => Some(artist.id),
                                        Err(_) => None,
                                    }
                                }
                            }
                        }
                        None => None,
                    };
                    let track_genre_id = match tag.genre() {
                        Some(genre_name) => {
                            use self::genres::dsl::*;
                            let new_genre = NewGenre {
                                name: String::from(genre_name),
                            };
                            let genre_result =
                                genres.filter(name.eq(genre_name)).first::<Genre>(&conn);
                            match genre_result {
                                Ok(genre) => Some(genre.id),
                                Err(_) => {
                                    diesel::insert_into(genres)
                                        .values(&new_genre)
                                        .execute(&conn)
                                        .expect("Error saving new genre");
                                    let genre_result =
                                        genres.filter(name.eq(genre_name)).first::<Genre>(&conn);
                                    match genre_result {
                                        Ok(genre) => Some(genre.id),
                                        Err(_) => None,
                                    }
                                }
                            }
                        }
                        None => None,
                    };
                    let track_track_number = tag.track();
                    let new_track = NewTrack {
                        name: track_name,
                        duration: track_duration.map(|td| td as i32),
                        album_id: track_album_id,
                        artist_id: track_artist_id,
                        genre_id: track_genre_id,
                        track_number: track_track_number.map(|ttn| ttn as i32),
                    };
                    use self::tracks::dsl::*;
                    diesel::insert_into(tracks)
                        .values(new_track)
                        .execute(&conn)
                        .expect("Error saving new track");
                    tracks.order(id.desc()).first::<Track>(&conn)
                }
                Err(_) => {
                    let content_type = field.content_disposition().unwrap();
                    let filename = content_type.get_filename().unwrap();
                    let path = Path::new(filename);
                    let file_stem = path.file_stem().unwrap();
                    let track_name = file_stem.to_str().unwrap();
                    let new_track = NewTrack {
                        name: String::from(track_name),
                        duration: None,
                        album_id: None,
                        artist_id: None,
                        genre_id: None,
                        track_number: None,
                    };
                    use self::tracks::dsl::*;
                    diesel::insert_into(tracks)
                        .values(new_track)
                        .execute(&conn)
                        .expect("Error saving new track");
                    tracks.order(id.desc()).first::<Track>(&conn)
                }
            })
            .unwrap();
        let filepath = format!("./static/{}.mp3", track.id());
        file.persist(filepath).unwrap();
        vec.push(track);
    }
    Ok(HttpResponse::Ok().json(vec))
}
