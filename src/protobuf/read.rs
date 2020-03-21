use crate::protobuf::BufReader;
use std::net::TcpStream;
use std::fmt::Write;
use std::borrow::Borrow;
use std::io::Read;

impl BufReader {
    pub fn byte(&mut self) -> u8 {
        let mut res: [u8; 1] = [0];
        self.data.read_exact(&mut res).unwrap();
        res[0]
    }

    pub fn ushort(&mut self) -> u16 {
        let mut number = (self.byte() as u16) << 8;
        number += self.byte() as u16;
        number
    }

    pub fn varint(&mut self) -> i32 {
        let mut res: u32 = 0; // u32 used to ignore sign
        let mut idx = 0;
        loop {
            let byte = self.byte();
            if byte >> 7 == 1 {
                // Remove the 7 last bits
                let mut tmp = byte as u32 & 0x7f;
                tmp <<= 7 * idx; // Shift by the number of byte already iterated
                res += tmp;
                idx += 1;
            }
            else {
                let mut tmp = byte as u32;
                tmp <<= 7 * idx;
                res += tmp;
                break;
            }
        }
        res as i32
    }

    pub fn string(&mut self) -> String {
        let len = self.varint();
        let mut res = String::new();
        for _idx in 0..len {
            res.write_char(self.byte() as char).unwrap();
        }
        res
    }

    pub fn get_stream(&mut self) -> &TcpStream {
        self.data.borrow()
    }
}
