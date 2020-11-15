CREATE TABLE tracks (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
	name TEXT NOT NULL,
	duration INTEGER NOT NULL,
	album_id INTEGER,
	artist_id INTEGER,
	genre_id INTEGER,
	track_number INTEGER,
	FOREIGN KEY(album_id) REFERENCES albums(id) ON UPDATE CASCADE ON DELETE SET NULL,
	FOREIGN KEY(artist_id) REFERENCES artists(id) ON UPDATE CASCADE ON DELETE SET NULL,
	FOREIGN KEY(genre_id) REFERENCES genres(id) ON UPDATE CASCADE ON DELETE SET NULL
)