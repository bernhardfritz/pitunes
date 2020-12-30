use std::{sync::Arc, time::Duration};

use actix_web::{web, Error, HttpRequest, HttpResponse};
use juniper::{execute, DefaultScalarValue, InputValue, Value, Variables};

use crate::graphql_schema::{RequestContext, Schema};

#[get("/playlists/{playlist_id}.m3u8")]
async fn get_playlist(
    st: web::Data<Arc<Schema>>,
    ctx: web::Data<RequestContext>,
    req: HttpRequest,
    web::Path(playlist_id): web::Path<String>,
) -> Result<HttpResponse, Error> {
    let body = {
        let query = r#"query PlaylistTracksQuery($id: ID!) {
  playlist(id: $id) {
    id
    name
    tracks {
      id
      name
      duration
      album {
        id
        name
      }
      artist {
        id
        name
      }
      genre {
        id
        name
      }
      trackNumber
    }
  }
}"#;
        let mut variables = Variables::<DefaultScalarValue>::new();
        variables.insert(
            String::from("id"),
            InputValue::Scalar(DefaultScalarValue::String(playlist_id)),
        );
        let mut lines = Vec::<String>::new();
        lines.push(String::from("#EXTM3U"));
        if let Ok((value, _errors)) =
            execute(query, Some("PlaylistTracksQuery"), &st, &variables, &ctx)
        {
            if let Value::Object(playlists) = value {
                if let Some(Value::Object(playlist)) = playlists.get_field_value("playlist") {
                    if let Some(Value::List(tracks)) = playlist.get_field_value("tracks") {
                        for track in tracks {
                            if let Value::Object(track) = track {
                                if let (
                                    Some(Value::Scalar(DefaultScalarValue::String(track_id))),
                                    Some(Value::Scalar(DefaultScalarValue::String(track_name))),
                                    Some(Value::Scalar(DefaultScalarValue::Int(duration))),
                                    artist,
                                ) = (
                                    track.get_field_value("id"),
                                    track.get_field_value("name"),
                                    track.get_field_value("duration"),
                                    track.get_field_value("artist"),
                                ) {
                                    if let Some(Value::Object(artist)) = artist {
                                        if let Some(Value::Scalar(DefaultScalarValue::String(
                                            artist_name,
                                        ))) = artist.get_field_value("name")
                                        {
                                            lines.push(format!(
                                                "#EXTINF:{},{} - {}",
                                                Duration::from_millis(*duration as u64).as_secs(),
                                                artist_name,
                                                track_name
                                            ));
                                        } else {
                                            lines.push(format!(
                                                "#EXTINF:{},{}",
                                                Duration::from_millis(*duration as u64).as_secs(),
                                                track_name
                                            ));
                                        }
                                    } else {
                                        lines.push(format!(
                                            "#EXTINF:{},{}",
                                            Duration::from_millis(*duration as u64).as_secs(),
                                            track_name
                                        ));
                                    }
                                    lines.push(
                                        req.url_for("get_track", &[&track_id[..]])?.to_string(),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        lines.push(String::new());
        lines.join("\n")
    };
    Ok(HttpResponse::Ok().body(body))
}
