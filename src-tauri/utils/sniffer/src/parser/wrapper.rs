#[derive(Debug, Clone)]
pub struct DataWrapper {
    pub data: Vec<u8>,
    pub pos: usize,
}

impl DataWrapper {
    pub fn new(data: Vec<u8>) -> Self {
        Self { data, pos: 0 }
    }

    pub fn get_remaining(&self) -> &[u8] {
        &self.data[self.pos..]
    }

    pub fn remaining(&self) -> usize {
        self.data.len() - self.pos
    }

    pub fn set_pos(&mut self, pos: usize) {
        self.pos = pos;
    }

    pub fn append_data(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn reset(&mut self) {
        self.pos = 0;
        self.data.clear();
    }

    pub fn read_byte(&mut self) -> u8 {
        let value = self.data[self.pos];
        self.pos += 1;
        value
    }

    pub fn read_int(&mut self) -> u32 {
        let value = u32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        value
    }

    pub fn read_var_int(&mut self) -> u32 {
        let mut value = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_byte();
            value |= ((byte & 0x7F) as u32) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        value
    }

    pub fn read_short(&mut self) -> i16 {
        let value = i16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        value
    }
    pub fn read_unsigned_short(&mut self) -> u16 {
        let value = u16::from_be_bytes([self.data[self.pos], self.data[self.pos + 1]]);
        self.pos += 2;
        value
    }

    pub fn read_float(&mut self) -> f32 {
        let value = f32::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
        ]);
        self.pos += 4;
        value
    }

    pub fn read_var_short(&mut self) -> u16 {
        let mut value = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_byte();
            value |= ((byte & 0x7F) as u16) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        value
    }

    pub fn read_var_long(&mut self) -> u64 {
        let mut value = 0;
        let mut shift = 0;
        loop {
            let byte = self.read_byte();
            value |= ((byte & 0x7F) as u64) << shift;
            shift += 7;
            if byte & 0x80 == 0 {
                break;
            }
        }
        value
    }

    pub fn read_utf(&mut self) -> String {
        let len = self.read_unsigned_short() as usize;
        if self.pos + len > self.data.len() {
            panic!(
                "Invalid utf length: {} + {} > {}",
                self.pos,
                len,
                self.data.len()
            );
        }
        let value = String::from_utf8(self.data[self.pos..self.pos + len].to_vec()).unwrap();
        self.pos += len;
        value
    }

    pub fn read_double(&mut self) -> f64 {
        let value = f64::from_be_bytes([
            self.data[self.pos],
            self.data[self.pos + 1],
            self.data[self.pos + 2],
            self.data[self.pos + 3],
            self.data[self.pos + 4],
            self.data[self.pos + 5],
            self.data[self.pos + 6],
            self.data[self.pos + 7],
        ]);
        self.pos += 8;
        value
    }

    pub fn read(&mut self, len: usize) -> Vec<u8> {
        if self.pos + len > self.data.len() {
            panic!(
                "Out of bounds: {} + {} > {}",
                self.pos,
                len,
                self.data.len()
            );
        }
        let mut value = Vec::with_capacity(len);
        for i in 0..len {
            value.push(self.data[self.pos + i]);
        }
        self.pos += len;
        value
    }
}
