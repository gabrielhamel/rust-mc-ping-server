pub mod write;
pub mod read;

pub struct BufWriter {
    pub data: Vec<u8>
}

pub struct BufReader {
    data: Vec<u8>,
    iterator: usize
}

impl BufWriter {
    pub fn new() -> Self {
        BufWriter {
            data: Vec::new()
        }
    }
}

impl From<Vec<u8>> for BufReader {
    fn from(buf: Vec<u8>) -> Self {
        BufReader {
            data: buf,
            iterator: 0
        }
    }
}
