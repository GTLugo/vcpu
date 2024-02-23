use strum::Display;

use crate::{core::cpu::Cpu, error::CpuError};

#[derive(Debug)]
pub struct Instruction {
  pub opcode: OpCode,
  pub mode: AddressingMode,
  pub cycles: u8,
}

#[derive(Debug, Display)]
pub enum AddressingMode {
  Accumulator,
  Implied,
  Immediate(u8),
  Relative(u8),

  ZeroPage(u8),
  ZeroPageX(u16),
  ZeroPageY(u16),

  Absolute(u16),
  AbsoluteX(u16),
  AbsoluteY(u16),

  Indirect(u16),
  IndirectX(u16),
  IndirectY(u16),
}

impl AddressingMode {
  pub fn accumulator(_cpu: &mut Cpu) -> Result<Self, CpuError> {
    Ok(Self::Accumulator)
  }

  pub fn implied(_cpu: &mut Cpu) -> Result<Self, CpuError> {
    Ok(Self::Implied)
  }

  pub fn immediate(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let value = cpu.fetch()?;
    Ok(AddressingMode::Immediate(value))
  }

  pub fn relative(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let value = cpu.fetch()?;
    Ok(AddressingMode::Relative(value))
  }

  pub fn zero_page(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let value = cpu.fetch()?;
    Ok(AddressingMode::ZeroPage(value))
  }

  pub fn zero_page_x(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::ZeroPageX(value))
  }

  pub fn zero_page_y(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::ZeroPageY(value))
  }

  pub fn absolute(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::Absolute(value))
  }

  pub fn absolute_x(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::AbsoluteX(value))
  }

  pub fn absolute_y(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::AbsoluteY(value))
  }

  pub fn indirect(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::Indirect(value))
  }

  pub fn indirect_x(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::IndirectX(value))
  }

  pub fn indirect_y(cpu: &mut Cpu) -> Result<Self, CpuError> {
    let first = cpu.fetch()?;
    let second = cpu.fetch()?;
    let value = u16::from_le_bytes([first, second]);
    Ok(AddressingMode::IndirectY(value))
  }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Display)]
pub enum OpCode {
  ADC,
  AND,
  ASL,
  BCC,
  BCS,
  BEQ,
  BIT,
  BMI,
  BNE,
  BPL,
  BRK,
  BVC,
  BVS,
  CLC,
  CLD,
  CLI,
  CLV,
  CMP,
  CPX,
  CPY,
  DEC,
  DEX,
  DEY,
  EOR,
  INC,
  INX,
  INY,
  JMP,
  JSR,
  LDA,
  LDX,
  LDY,
  LSR,
  NOP,
  ORA,
  PHA,
  PHP,
  PLA,
  PLP,
  ROL,
  ROR,
  RTI,
  RTS,
  SBC,
  SEC,
  SED,
  SEI,
  STA,
  STX,
  STY,
  TAX,
  TAY,
  TSX,
  TXA,
  TXS,
  TYA,
}
