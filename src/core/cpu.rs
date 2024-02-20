use std::fmt::{Debug, Formatter};
use std::io::{stdout, Write};
use std::time::Duration;

use enumflags2::{bitflags, BitFlag, BitFlags};

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
  Interrupt = 1 << 2,
  Decimal = 1 << 3,
  Break = 1 << 4,
  Unused = 1 << 5,
  Overflow = 1 << 6,
  Negative = 1 << 7,
}

pub struct Cpu {
  data_bus: Bus,

  pub(crate) a: u8,
  pub(crate) x: u8,
  pub(crate) y: u8,
  pub(crate) stack_ptr: u8,
  pub(crate) prog_counter: u16,
  pub(crate) status: BitFlags<StatusFlags>,

  clock_speed: f64,
  cycles: u8,
}

impl Debug for Cpu {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(
      f,
      "a: {:#04X} | x: {:#04X} | y: {:#04X} | stack_ptr: {:#04X} | prog_counter: {:#06X} | cycles: {}",
      self.a, self.x, self.y, self.stack_ptr, self.prog_counter, self.cycles
    )
  }
}

impl Cpu {
  pub const INTERRUPT_ADDRESS: u16 = 0xFFFE;
  pub const RESET_ADDRESS: u16 = 0xFFFC;
  pub const STACK_POINTER_BASE_ADDRESS: u16 = 0x0100;

  pub fn new(clock_speed: f64) -> Self {
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
      clock_speed,
      cycles: 0,
    }
  }

  pub fn connect(&mut self, item: impl BusItem + 'static) {
    self.data_bus.connect(item)
  }

  pub fn clock(&mut self) -> Result<(), CpuError> {
    if self.cycles == 0 {
      let state_before = format!("{self:?}");
      let opcode = self.fetch()?;
      let instruction = self.decode(opcode)?;
      let instr_string = format!("  |\n  +--> {instruction:?}");
      self.cycles += self.execute(instruction)?;
      let mut lock = stdout().lock();
      writeln!(lock, "Before: [{state_before}]\nAfter: [{self:?}]\n{instr_string}\n").unwrap();
      // std::io::stdout().write_fmt(format_args!()).flush().expect("a");
    }

    self.cycles -= 1;

    Ok(())
  }

  pub fn read(&self, address: u16) -> u8 {
    self.data_bus.read(address, false)
  }

  pub fn write(&mut self, address: u16, value: u8) {
    self.data_bus.write(address, value)
  }

  pub fn is_flag(&self, flag: StatusFlags) -> bool {
    self.status.contains(flag)
  }

  pub fn set_flag(&mut self, flag: StatusFlags, set: bool) {
    self.status.set(flag, set);
  }

  pub fn reset(&mut self) {
    self.a = 0;
    self.x = 0;
    self.y = 0;
    self.stack_ptr = 0xFD;
    self.status = StatusFlags::empty() | StatusFlags::Unused;

    let lo = self.read(Self::RESET_ADDRESS);
    let hi = self.read(Self::RESET_ADDRESS + 1);
    self.prog_counter = u16::from_le_bytes([lo, hi]);

    self.cycles = 8;
  }

  pub fn interrupt(&mut self) {
    if !self.is_flag(StatusFlags::Interrupt) {
      let [lo, hi] = self.prog_counter.to_le_bytes();
      self.push(hi);
      self.push(lo);

      self.set_flag(StatusFlags::Break, false);
      self.set_flag(StatusFlags::Unused, true);
      self.set_flag(StatusFlags::Interrupt, true);
      self.push(self.status.bits());

      let lo = self.read(Self::INTERRUPT_ADDRESS);
      let hi = self.read(Self::INTERRUPT_ADDRESS + 1);
      self.prog_counter = u16::from_le_bytes([lo, hi]);

      self.cycles = 7;
    }
  }

  pub fn non_maskable_interrupt(&mut self) {
    let [lo, hi] = self.prog_counter.to_le_bytes();
    self.push(hi);
    self.push(lo);

    self.set_flag(StatusFlags::Break, false);
    self.set_flag(StatusFlags::Unused, true);
    self.set_flag(StatusFlags::Interrupt, true);
    self.push(self.status.bits());

    let lo = self.read(Self::INTERRUPT_ADDRESS);
    let hi = self.read(Self::INTERRUPT_ADDRESS + 1);
    self.prog_counter = u16::from_le_bytes([lo, hi]);

    self.cycles = 8;
  }

  pub fn fetch(&mut self) -> Result<u8, CpuError> {
    let value = self.read(self.prog_counter);
    self.prog_counter += 1;
    Ok(value)
  }

  pub fn pop(&mut self) -> u8 {
    self.stack_ptr += 1;
    self.read(Self::STACK_POINTER_BASE_ADDRESS + self.stack_ptr as u16)
  }

  pub fn push(&mut self, value: u8) {
    self.write(Self::STACK_POINTER_BASE_ADDRESS + self.stack_ptr as u16, value);
    self.stack_ptr -= 1;
  }
}

impl Iterator for Cpu {
  type Item = ();

  fn next(&mut self) -> Option<Self::Item> {
    loop {
      if let Err(error) = self.clock() {
        eprintln!("{error}");
        return None;
      };

      std::thread::sleep(Duration::from_secs_f64(1.0 / self.clock_speed));

      if self.cycles == 0 {
        break;
      }
    }

    Some(())
  }
}
