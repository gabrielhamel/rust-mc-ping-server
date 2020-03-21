pub mod read;
pub mod write;

use std::net::TcpStream;

pub struct BufReader {
    data: Box<TcpStream>
}

impl From<TcpStream> for BufReader {
    fn from(data: TcpStream) -> Self {
        BufReader {
            data: std::boxed::Box::new(data)
        }
    }
}

pub struct BufWriter {
    pub data: Vec<u8>
}

impl BufWriter {
    pub fn new() -> Self {
        BufWriter {
            data: Vec::new()
        }
    }
}
