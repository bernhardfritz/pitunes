use std::{
    io::{Error, ErrorKind, Read, Seek, SeekFrom},
    str::FromStr,
};

pub struct HttpStreamReader {
    url: String,
    api_key: String,
    client: reqwest::blocking::Client,
    start: u64,
    end: u64,
}

impl HttpStreamReader {
    pub fn new(url: String, api_key: String) -> Self {
        let client = reqwest::blocking::Client::new();
        let res = client.head(&url).bearer_auth(&api_key[..]).send().unwrap();
        let length = res
            .headers()
            .get(reqwest::header::CONTENT_LENGTH)
            .ok_or("response doesn't include the content length")
            .unwrap();
        let length = u64::from_str(length.to_str().unwrap())
            .map_err(|_| "invalid Content-Length header")
            .unwrap();
        HttpStreamReader {
            url,
            api_key,
            client,
            start: 0,
            end: length - 1,
        }
    }
}

impl Read for HttpStreamReader {
    fn read(&mut self, mut buf: &mut [u8]) -> Result<usize, Error> {
        if self.start > self.end {
            Ok(0)
        } else {
            let prev_start = self.start;
            self.start += std::cmp::min(buf.len() as u64, self.end - self.start + 1);
            // NOTE(unwrap): `HeaderValue::from_str` will fail only if the value is not made
            // of visible ASCII characters. Since the format string is static and the two
            // values are integers, that can't happen.
            let range = reqwest::header::HeaderValue::from_str(&format!(
                "bytes={}-{}",
                prev_start,
                self.start - 1
            ))
            .unwrap();
            let mut res = self
                .client
                .get(&self.url)
                .bearer_auth(&self.api_key[..])
                .header(reqwest::header::RANGE, range)
                .send()
                .unwrap();

            let status = res.status();
            if status == reqwest::StatusCode::OK || status == reqwest::StatusCode::PARTIAL_CONTENT {
                Ok(std::io::copy(&mut res, &mut buf).unwrap() as usize)
            } else {
                Err(Error::new(
                    ErrorKind::Other,
                    format!("Unexpected server response: {}", status),
                ))
            }
        }
    }
}

impl Seek for HttpStreamReader {
    fn seek(&mut self, pos: SeekFrom) -> Result<u64, Error> {
        match pos {
            SeekFrom::Start(offset) => {
                self.start = offset;
                Ok(self.start)
            }
            SeekFrom::End(offset) => {
                if offset.is_negative() {
                    let offset_abs = offset.abs() as u64;
                    if self.end >= offset_abs {
                        self.start = self.end - offset_abs;
                        Ok(self.start)
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidInput,
                            "It's an error to seek before byte 0",
                        ))
                    }
                } else {
                    self.start = self.end + offset as u64;
                    Ok(self.start)
                }
            }
            SeekFrom::Current(offset) => {
                if offset.is_negative() {
                    let offset_abs = offset.abs() as u64;
                    if self.start >= offset_abs {
                        self.start = self.start - offset_abs;
                        Ok(self.start)
                    } else {
                        Err(Error::new(
                            ErrorKind::InvalidInput,
                            "It's an error to seek before byte 0",
                        ))
                    }
                } else {
                    self.start = self.start + offset as u64;
                    Ok(self.start)
                }
            }
        }
    }
}
