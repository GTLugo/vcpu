use enumflags2::{bitflags, BitFlags};

use crate::{
  core::bus::{Bus, BusItem},
  error::CpuError,
};

#[bitflags]
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum StatusFlags {
  Carry = 1 << 0,
  Zero = 1 << 1,
  DisableInterrupts = 1 << 2,
  DecimalMode = 1 << 3,
  Break = 1 << 4,
  Unused = 1 << 5,
  Overflow = 1 << 6,
  Negative = 1 << 7,
}

#[derive(Default)]
pub struct Cpu {
  data_bus: Bus,

  pub(crate) a: u8,
  pub(crate) x: u8,
  pub(crate) y: u8,
  pub(crate) stack_ptr: u8,
  pub(crate) prog_counter: u16,
  pub(crate) status: BitFlags<StatusFlags>,

  cycles: u8,
}

impl Cpu {
  pub fn new() -> Self {
    let data_bus = Bus::new();
    let status = Default::default();

    Self {
      data_bus,
      a: 0,
      x: 0,
      y: 0,
      stack_ptr: 0,
      prog_counter: 0,
      status,
      cycles: 0,
    }
  }

  pub fn read(&self, address: u16) -> u8 {
    self.data_bus.read(address, false)
  }

  pub fn write(&mut self, address: u16, value: u8) {
    self.data_bus.write(address, value)
  }

  pub fn connect(&mut self, item: impl BusItem + 'static) {
    self.data_bus.connect(item)
  }

  pub fn is_flag(&self, flag: StatusFlags) -> bool {
    self.status.contains(flag)
  }

  pub fn set_flag(&mut self, flag: StatusFlags, set: bool) {
    self.status.set(flag, set);
  }

  pub fn clock(&mut self) -> Result<(), CpuError> {
    if self.cycles == 0 {
      let opcode = self.fetch()?;
      let instruction = self.decode(opcode)?;
      self.cycles += self.execute(instruction)?;
    }

    Ok(())
  }

  pub fn reset(&mut self) {}

  pub fn interrupt(&mut self) {}

  pub fn non_maskable_interrupt(&mut self) {}

  pub(crate) fn fetch(&mut self) -> Result<u8, CpuError> {
    let value = self.read(self.prog_counter);
    self.prog_counter += 1;
    Ok(value)
  }
}
