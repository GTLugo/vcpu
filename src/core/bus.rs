use std::ops::RangeInclusive;

// TODO: Try invert the dependency on bus so Cpu and Ram hold channels to/from
// Bus

#[derive(Default)]
pub struct Bus {
  items: Vec<Box<dyn BusItem>>,
}

impl Bus {
  pub fn new() -> Self {
    let items = Default::default();

    Self { items }
  }

  pub fn read(&self, address: u16, read_only: bool) -> u8 {
    for item in self.items.iter() {
      if item.address_range().contains(&address) {
        return item.read(address, read_only);
      }
    }

    0
  }

  pub fn write(&mut self, address: u16, value: u8) {
    for item in self.items.iter_mut() {
      if item.address_range().contains(&address) {
        return item.write(address, value);
      }
    }
  }

  pub fn connect(&mut self, item: impl BusItem + 'static) {
    self.items.push(Box::new(item));
  }
}

pub trait BusItem {
  fn address_range(&self) -> &RangeInclusive<u16>;

  fn read(&self, address: u16, read_only: bool) -> u8;

  fn write(&mut self, address: u16, value: u8);
}
