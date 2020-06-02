use std::io::Read;

const DEFAULT_BUF_SIZE: usize = 8 * 1024;

pub struct Chunker<R: Read> {
    reader: R,
}

impl<R: Read> Chunker<R> {
    pub fn new(reader: R) -> Self {
        Self { reader }
    }
}

impl<R: Read> Iterator for Chunker<R> {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Vec<u8>> {
        let mut buf = vec![0u8; DEFAULT_BUF_SIZE];
        let n = self.reader.read(&mut buf[..]).unwrap();
        if n == 0 {
            return None;
        }
        buf.truncate(n);
        Some(buf)
    }
}
