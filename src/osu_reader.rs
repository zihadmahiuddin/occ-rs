pub struct OsuReader {
  buffer: Vec<u8>,
  offset: usize,
}

impl OsuReader {
  pub fn new(buffer: Vec<u8>) -> OsuReader {
    OsuReader {
      buffer,
      offset: 0,
    }
  }

  pub fn read_u8(&mut self) -> u8 {
    let value = self.buffer[self.offset];
    self.offset += 1;
    return value
  }

  pub fn read_i32(&mut self) -> i32 {
    let mut i = 0 as usize;
    let mut result = 0;
    while i < 4 {
      let x = self.buffer[self.offset + i] as i32;
      result += (x) << (i * 8);
      i += 1;
    }
    self.offset += 4;
    return result
  }

  fn read_uleb128(&mut self) -> i32 {
    let mut result = 0;
    let mut shift = 0;

    loop {
      let byte = self.read_u8();
      result |= ((byte & 0x7f) as i32) << shift;

      if ((byte & 0x80) >> 7) == 0 {
        break;
      }

      shift += 7;
    }

    return result
  }

  pub fn read_string(&mut self) -> String {
    let empty_indicator = self.read_u8();
    if empty_indicator == 0 {
      return String::new();
    }

    let length = self.read_uleb128();

    let mut result = String::new();
    let mut i = 0;
    while i < length as usize {
      result.push(self.read_u8() as char);
      i += 1;
    }
    return result
  }
}
