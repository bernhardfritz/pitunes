CREATE TABLE users (
    username TEXT NOT NULL PRIMARY KEY,
    password BLOB NOT NULL
);
INSERT INTO users VALUES ('admin', x'5e884898da28047151d0e56f8dc6292773603d0d6aabbdd62a11ef721d1542d8'); -- password