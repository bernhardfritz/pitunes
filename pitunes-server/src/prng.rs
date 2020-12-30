use anyhow::Result;
use diesel::prelude::*;
use getrandom::getrandom;
use oorandom::Rand32;

use crate::{models::Prng, schema::prngs};

pub fn insert_prng_if_not_exists(conn: &SqliteConnection) -> Result<()> {
    let exists: bool =
        diesel::dsl::select(diesel::dsl::exists(prngs::table.find(1))).get_result(conn)?;
    if !exists {
        let mut buf = [0u8; 8];
        getrandom(&mut buf).unwrap();
        let seed = u64::from_le_bytes(buf);
        let rand32 = Rand32::new(seed);
        let prng: Prng = rand32.into();
        diesel::insert_into(prngs::table)
            .values(&prng)
            .execute(conn)?;
    }
    Ok(())
}

pub fn rand_i32(conn: &SqliteConnection) -> Result<i32> {
    let prng = prngs::table.find(1).get_result::<Prng>(conn)?;
    let mut rand32: Rand32 = prng.into();
    let i = rand32.rand_i32();
    let prng: Prng = rand32.into();
    diesel::update(prngs::table).set(&prng).execute(conn)?;
    Ok(i)
}
