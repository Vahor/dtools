use tracing::debug;

#[derive(Debug, Clone)]
pub struct DataWrapper {
    pub data: Vec<u8>,
    pub pos: usize,
}

/// Adapted from com.ankamagames.jerakine.network.CustomDataWrapper
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

    pub fn extend_from_slice(&mut self, data: &[u8]) {
        self.data.extend_from_slice(data);
    }

    pub fn debug_remaining(&self) {
        // print body as string
        let body = self
            .get_remaining()
            .iter()
            .map(|b| *b as char)
            .collect::<String>();
        debug!("Body: {}", body);
        debug!("Remaining: {:?}", self.get_remaining());
    }

    pub fn reorder(&mut self, buffer: Vec<u8>) {
        let buffer_len = buffer.len();
        let cut_off = self.data.len().saturating_sub(buffer_len) as usize;

        let mut new_data = Vec::with_capacity(self.data.len() + buffer_len);
        for i in 0..cut_off {
            new_data.push(self.data[i]);
        }
        for b in buffer {
            new_data.push(b);
        }
        for i in cut_off..self.data.len() {
            new_data.push(self.data[i]);
        }
        self.data = new_data;
    }

    pub fn clear(&mut self) {
        self.pos = 0;
        self.data.clear();
    }

    pub fn read_byte(&mut self) -> u8 {
        let value = self.data[self.pos];
        let value = u8::from_be_bytes([value]); // TODO: check if needed
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
        for i in (0..32).step_by(7) {
            let byte = self.read_byte();
            value |= ((byte & 0x7f) as u32) << i;
            if byte & 0x80 == 0 {
                return value;
            }
        }
        panic!("Too much data");
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
        for i in (0..16).step_by(7) {
            let byte = self.read_byte();
            value += ((byte & 0x7f) as u16) << i;
            if byte & 0x80 == 0 {
                return value;
            }
        }
        panic!("Too much data");
    }

    pub fn read_var_long(&mut self) -> u64 {
        let mut value = 0;
        for i in (0..64).step_by(7) {
            let byte = self.read_byte();
            value |= ((byte & 0x7f) as u64) << i;
            if byte & 0x80 == 0 {
                return value;
            }
        }
        panic!("Too much data");
    }

    pub fn read_utf(&mut self) -> String {
        let len = self.read_unsigned_short() as usize;
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
}
