use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params;
use std::fs::File;
use std::io::prelude::*;

pub fn migrate(db: &Pool<SqliteConnectionManager>) -> std::io::Result<()> {
    let mut conn = db.get().unwrap();
    let mut user_version = conn
        .query_row("PRAGMA user_version", params![], |row| row.get::<_, u32>(0))
        .unwrap();
    while let Ok(mut file) = File::open(format!("migrations/{}.sql", user_version + 1)) {
        let mut buf = String::new();
        file.read_to_string(&mut buf)?;
        let transaction = conn.transaction().unwrap();
        transaction.execute_batch(&buf[..]).unwrap();
        user_version = user_version + 1;
        let sql = format!("PRAGMA user_version={}", user_version);
        transaction.execute(&sql[..], params![]).unwrap();
        transaction.commit().unwrap();
    }
    Ok(())
}
