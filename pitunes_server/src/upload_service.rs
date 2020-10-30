use std::{
    io::{BufReader, BufWriter, Seek, SeekFrom, Write},
    path::Path,
};

use actix_multipart::Multipart;
use actix_web::{web, Error, HttpResponse};
use diesel::prelude::*;
use futures::{StreamExt, TryStreamExt};

use crate::{
    chunker::Chunker,
    graphql_schema::RequestContext,
    models::{
        Album, AlbumInput, Artist, ArtistInput, Genre, GenreInput, Track, TrackInputInternal,
    },
    schema::{albums, artists, genres, tracks},
};

#[post("/upload")]
async fn post_upload(
    context: web::Data<RequestContext>,
    mut payload: Multipart,
) -> Result<HttpResponse, Error> {
    let mut vec = Vec::new();
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
        if let Ok(track) = conn.transaction::<_, diesel::result::Error, _>(|| {
            let track_input = if let Ok(tag) = id3::Tag::read_from(tf2) {
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
                        diesel::insert_into(albums::table)
                            .values(AlbumInput {
                                name: String::from(album_name),
                            })
                            .execute(&conn)?;
                        if let Ok(album) =
                            albums::table.order(albums::id.desc()).first::<Album>(&conn)
                        {
                            Some(album.id)
                        } else {
                            None
                        }
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
                        diesel::insert_into(artists::table)
                            .values(ArtistInput {
                                name: String::from(artist_name),
                            })
                            .execute(&conn)?;
                        if let Ok(artist) = artists::table
                            .order(artists::id.desc())
                            .first::<Artist>(&conn)
                        {
                            Some(artist.id)
                        } else {
                            None
                        }
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
                        diesel::insert_into(genres::table)
                            .values(GenreInput {
                                name: String::from(genre_name),
                            })
                            .execute(&conn)?;
                        if let Ok(genre) =
                            genres::table.order(genres::id.desc()).first::<Genre>(&conn)
                        {
                            Some(genre.id)
                        } else {
                            None
                        }
                    }
                } else {
                    None
                };
                let track_track_number = tag.track();
                TrackInputInternal {
                    name: track_name,
                    duration: track_duration,
                    album_id: track_album_id,
                    artist_id: track_artist_id,
                    genre_id: track_genre_id,
                    track_number: track_track_number.map(|ttn| ttn as i32),
                }
            } else {
                let content_type = field.content_disposition().unwrap();
                let filename = content_type.get_filename().unwrap();
                let path = Path::new(filename);
                let file_stem = path.file_stem().unwrap();
                let track_name = String::from(file_stem.to_str().unwrap());
                let track_duration = duration.unwrap().as_millis() as i32;
                TrackInputInternal {
                    name: track_name,
                    duration: track_duration,
                    album_id: None,
                    artist_id: None,
                    genre_id: None,
                    track_number: None,
                }
            };
            diesel::insert_into(tracks::table)
                .values(track_input)
                .execute(&conn)?;
            tracks::table.order(tracks::id.desc()).first::<Track>(&conn)
        }) {
            let reader = BufReader::new(tf);
            let mut chunker = Chunker::new(reader);
            let filepath = format!("./static/{}.mp3", track.id());
            // File::create is blocking operation, use threadpool
            let f = web::block(|| std::fs::File::create(filepath)).await?;
            let mut writer = BufWriter::new(f);
            while let Some(chunk) = chunker.next() {
                // filesystem operations are blocking, we have to use threadpool
                writer = web::block(move || writer.write_all(&chunk).map(|_| writer)).await?;
            }
            vec.push(track);
        }
    }
    Ok(HttpResponse::Ok().json(vec))
}

#[get("/upload")]
fn get_upload() -> HttpResponse {
    let html = r#"
<!DOCTYPE html>
<html>
    <head>
        <title>Upload</title>
    </head>
    <body>
        <label for="api-key">API_KEY=</label>
        <input id="api-key" name="api-key" type="password"/>
        <form enctype="multipart/form-data" method="post" name="upload-form">
            <input type="file" multiple name="file"/>
            <input type="submit" value="Upload">
        </form>
        <label for="output"></label>
        <textarea id="output" name="output" disabled></textarea>
        <script>
            const apiKey = document.getElementById('api-key');
            const uploadForm = document.forms.namedItem('upload-form');
            const outputTextArea = document.getElementById('output');
            uploadForm.addEventListener('submit', event => {
                const oData = new FormData(uploadForm);
                const oReq = new XMLHttpRequest();
                oReq.open('POST', 'upload', true);
                oReq.setRequestHeader('Authorization', `Bearer ${apiKey.value}`);
                oReq.onload = event => {
                    outputTextArea.value = oReq.responseText;
                };
                oReq.send(oData);
                event.preventDefault();
            }, false);
        </script>
    </body>
</html>
"#;
    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(html)
}
