use std::convert::{TryFrom, TryInto};

use anyhow::Result;
use base64;

pub struct ExternalId(pub juniper::ID);

impl From<i32> for ExternalId {
    fn from(id: i32) -> Self {
        ExternalId(juniper::ID::from(base64::encode_config(
            id.to_le_bytes(),
            base64::URL_SAFE_NO_PAD,
        )))
    }
}

impl TryFrom<ExternalId> for i32 {
    type Error = anyhow::Error;

    fn try_from(external_id: ExternalId) -> Result<Self> {
        let v = base64::decode_config(&external_id.0[..], base64::URL_SAFE_NO_PAD)?;
        let b = v.try_into().unwrap();
        let i = i32::from_le_bytes(b);
        Ok(i)
    }
}

#[test]
fn it_works() {
    let id: i64 = 882286793;
    let encoded = base64::encode_config(id.to_le_bytes(), base64::URL_SAFE_NO_PAD);
    let decoded = base64::decode_config(encoded, base64::URL_SAFE_NO_PAD).unwrap();
    let decoded: [u8; 8] = decoded.try_into().unwrap();
    let decoded = i64::from_le_bytes(decoded);
    assert_eq!(decoded, id);
}
