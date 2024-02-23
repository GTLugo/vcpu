use std::{
  env,
  fs,
  fs::File,
  io::{Read, Seek, SeekFrom, Write},
  ops::RangeInclusive,
};

use crate::core::bus::BusItem;

pub struct RamFile {
  address_range: RangeInclusive<u16>,
  file: File,
}

impl RamFile {
  pub fn new(address_range: RangeInclusive<u16>) -> Self {
    let exe = env::current_exe().unwrap_or_default();
    let exe_dir = exe.parent().unwrap();
    let tmp_dir = exe_dir.join("tmp");
    fs::create_dir_all(&tmp_dir).expect("failed to create tmp_dir");
    let file_path = tmp_dir.join("ram");

    let mut file = File::options()
      .write(true)
      .read(true)
      .create(true)
      .truncate(false)
      .open(&file_path)
      .unwrap();
    let metadata = fs::metadata(&file_path).unwrap();

    if metadata.len() == 0 {
      file.set_len(address_range.len() as u64).unwrap();
      for _ in address_range.clone() {
        file.write_all(&[0]).unwrap();
      }
    }

    file.sync_all().unwrap();

    Self { address_range, file }
  }

  pub fn write_all(&mut self, address: u16, data: &[u8]) {
    self.file.seek(SeekFrom::Start(address as u64)).unwrap();
    self.file.write_all(data).unwrap();
  }
}

impl BusItem for RamFile {
  fn address_range(&self) -> &RangeInclusive<u16> {
    &self.address_range
  }

  fn read(&mut self, address: u16, _read_only: bool) -> u8 {
    let mut buffer = [0; 1];
    self.file.seek(SeekFrom::Start(address as u64)).unwrap();
    self.file.read_exact(&mut buffer).unwrap();
    buffer[0]
  }

  fn write(&mut self, address: u16, value: u8) {
    self.file.seek(SeekFrom::Start(address as u64)).unwrap();
    self.file.write_all(&[value]).unwrap();
    self.file.sync_all().unwrap();
  }
}
