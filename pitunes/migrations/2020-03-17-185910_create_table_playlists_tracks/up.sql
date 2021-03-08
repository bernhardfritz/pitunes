CREATE TABLE playlists_tracks (
	id INTEGER NOT NULL PRIMARY KEY,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    playlist_id INTEGER NOT NULL,
    track_id INTEGER NOT NULL,
    position INTEGER NOT NULL,
	FOREIGN KEY(playlist_id) REFERENCES playlists(id) ON UPDATE CASCADE ON DELETE CASCADE,
	FOREIGN KEY(track_id) REFERENCES tracks(id) ON UPDATE CASCADE ON DELETE CASCADE
)