use crate::protobuf::BufWriter;

impl BufWriter {
    pub fn byte(&mut self, byte: u8) {
        self.data.push(byte);
    }

    pub fn ushort(&mut self, ushort: u16) {
        self.data.push((ushort >> 8) as u8);
        self.data.push((ushort & 0xFF) as u8);
    }

    pub fn varint(&mut self, int: i32) {
        let mut number = int as u32; // To shift the sign with the LSR
        loop {
            let seven_first_bits = number & 0x7f;
            number >>= 7; // Prepare number for next round
            if number != 0 {
                self.data.push((seven_first_bits + 128) as u8); // Add an eighth bit if number is not finished
            }
            else {
                self.data.push(seven_first_bits as u8);
                break;
            }
        }
    }

    pub fn string(&mut self, string: &String) {
        self.varint(string.len() as i32);
        for letter in string.chars() {
            self.byte(letter as u8);
        }
    }

    pub fn concat(&mut self, buf: &mut Vec<u8>) {
        self.data.append(buf);
    }
}
