use ureq::Error;
use ureq::Response;
use std::thread;

use exponential_backoff::Backoff;
use ureq::{Request, SerdeValue};

const RETRIES: u32 = 8;

pub enum Payload {
    Empty,
    JSON(SerdeValue),
}

pub fn retry(req: Request, payload: Payload) -> Result<Response, Error> {
    let backoff = Backoff::new(RETRIES);
    for duration in &backoff {
        let res = match &payload {
            Payload::Empty => req.clone().call(),
            Payload::JSON(serde_value) => req.clone().send_json(serde_value.clone()),
        };
        match res {
            Ok(res) => return Result::Ok(res),
            Err(err) => match duration {
                Some(duration) => thread::sleep(duration),
                None => return Result::Err(err),
            },
        }
    }
    panic!()
}
