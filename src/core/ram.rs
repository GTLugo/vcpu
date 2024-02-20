use std::ops::RangeInclusive;

use crate::core::bus::BusItem;

pub struct Ram {
  address_range: RangeInclusive<u16>,
  data: Vec<u8>,
}

impl Ram {
  pub fn new(address_range: RangeInclusive<u16>) -> Self {
    let data = vec![0xEA; address_range.len()];

    Self { address_range, data }
  }

  pub fn write_all(&mut self, offset: u16, data: &[u8]) {
    let mut offset = offset as usize;
    for byte in data {
      self.data[offset] = *byte;
      offset += 1;
    }
  }
}

impl BusItem for Ram {
  fn address_range(&self) -> &RangeInclusive<u16> {
    &self.address_range
  }

  fn read(&mut self, address: u16, _read_only: bool) -> u8 {
    self.data.get(address as usize).cloned().unwrap_or_default()
  }

  fn write(&mut self, address: u16, value: u8) {
    match self.data.get_mut(address as usize) {
      None => (),
      Some(data) => *data = value,
    }
  }
}
