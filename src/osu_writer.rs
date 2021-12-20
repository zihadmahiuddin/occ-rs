pub struct OsuWriter {
  pub buffer: Vec<u8>,
  offset: usize,
}

impl OsuWriter {
  pub fn new() -> OsuWriter {
    OsuWriter {
      buffer: Vec::new(),
      offset: 0,
    }
  }

  pub fn write_u8(&mut self, value: u8) {
    self.buffer.push(value);
    self.offset += 1;
  }

  pub fn write_i32(&mut self, value: i32) {
    self.write_u8(value as u8);
    self.write_u8((value >> 8) as u8);
    self.write_u8((value >> 16) as u8);
    self.write_u8((value >> 24) as u8);
  }

  fn write_uleb128(&mut self, value: i32) {
    let mut value = value;
    loop {
      let byte = value & 0x7f;
      value >>= 7;
      if value != 0 {
        self.write_u8((byte | 0x80) as u8);
      } else {
        self.write_u8(byte as u8);
        break;
      }
    }
  }

  pub fn write_string(&mut self, value: &String) {
    if value.is_empty() {
      self.write_u8(0);
    } else {
      self.write_u8(11);
    }

    self.write_uleb128(value.len() as i32);
    for c in value.chars() {
      self.write_u8(c as u8);
    }
  }
}
