use crate::{
  core::{
    cpu::Cpu,
    instruction::{AddressingMode, Instruction, OpCode},
  },
  error::CpuError,
};

impl Cpu {
  pub(crate) fn decode(&mut self, opcode: u8) -> Result<Instruction, CpuError> {
    let (opcode, mode, cycles) = match opcode {
      // ADC
      0x69 => {
        let value = self.fetch()?;
        (OpCode::ADC, AddressingMode::Immediate(value), 2)
      }
      0x65 => {
        let value = self.fetch()?;
        (OpCode::ADC, AddressingMode::ZeroPage(value), 3)
      }
      0x75 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ADC, AddressingMode::ZeroPageX(value), 4)
      }
      0x6D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ADC, AddressingMode::Absolute(value), 4)
      }
      0x7D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ADC, AddressingMode::AbsoluteX(value), 4)
      }
      0x79 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ADC, AddressingMode::AbsoluteY(value), 4)
      }
      0x61 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ADC, AddressingMode::IndirectX(value), 6)
      }
      0x71 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ADC, AddressingMode::IndirectY(value), 5)
      }
      // AND
      0x29 => {
        let value = self.fetch()?;
        (OpCode::AND, AddressingMode::Immediate(value), 2)
      }
      0x25 => {
        let value = self.fetch()?;
        (OpCode::AND, AddressingMode::ZeroPage(value), 3)
      }
      0x35 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::AND, AddressingMode::ZeroPageX(value), 4)
      }
      0x2D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::AND, AddressingMode::Absolute(value), 4)
      }
      0x3D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::AND, AddressingMode::AbsoluteX(value), 4)
      }
      0x39 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::AND, AddressingMode::AbsoluteY(value), 4)
      }
      0x21 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::AND, AddressingMode::IndirectX(value), 6)
      }
      0x31 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::AND, AddressingMode::IndirectY(value), 5)
      }
      // ASL
      0x0A => (OpCode::ASL, AddressingMode::Accumulator, 2),
      0x06 => {
        let value = self.fetch()?;
        (OpCode::ASL, AddressingMode::ZeroPage(value), 5)
      }
      0x16 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ASL, AddressingMode::ZeroPageX(value), 6)
      }
      0x0E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ASL, AddressingMode::Absolute(value), 6)
      }
      0x1E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ASL, AddressingMode::AbsoluteX(value), 7)
      }
      // BIT
      0x24 => {
        let value = self.fetch()?;
        (OpCode::BIT, AddressingMode::ZeroPage(value), 3)
      }
      0x2C => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::BIT, AddressingMode::Absolute(value), 4)
      }
      // BRANCH
      0x10 => {
        let value = self.fetch()?;
        (OpCode::BPL, AddressingMode::Relative(value), 2)
      }
      0x30 => {
        let value = self.fetch()?;
        (OpCode::BMI, AddressingMode::Relative(value), 2)
      }
      0x50 => {
        let value = self.fetch()?;
        (OpCode::BVC, AddressingMode::Relative(value), 2)
      }
      0x70 => {
        let value = self.fetch()?;
        (OpCode::BVS, AddressingMode::Relative(value), 2)
      }
      0x90 => {
        let value = self.fetch()?;
        (OpCode::BCC, AddressingMode::Relative(value), 2)
      }
      0xB0 => {
        let value = self.fetch()?;
        (OpCode::BCS, AddressingMode::Relative(value), 2)
      }
      0xD0 => {
        let value = self.fetch()?;
        (OpCode::BNE, AddressingMode::Relative(value), 2)
      }
      0xF0 => {
        let value = self.fetch()?;
        (OpCode::BEQ, AddressingMode::Relative(value), 2)
      }
      // BREAK
      0x00 => (OpCode::BRK, AddressingMode::Implied, 7),
      // CMP
      0xC9 => {
        let value = self.fetch()?;
        (OpCode::CMP, AddressingMode::Immediate(value), 2)
      }
      0xC5 => {
        let value = self.fetch()?;
        (OpCode::CMP, AddressingMode::ZeroPage(value), 3)
      }
      0xD5 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CMP, AddressingMode::ZeroPageX(value), 4)
      }
      0xCD => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CMP, AddressingMode::Absolute(value), 4)
      }
      0xDD => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CMP, AddressingMode::AbsoluteX(value), 4)
      }
      0xD9 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CMP, AddressingMode::AbsoluteY(value), 4)
      }
      0xC1 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CMP, AddressingMode::IndirectX(value), 6)
      }
      0xD1 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CMP, AddressingMode::IndirectY(value), 5)
      }
      // CPX
      0xE0 => {
        let value = self.fetch()?;
        (OpCode::CPX, AddressingMode::Immediate(value), 2)
      }
      0xE4 => {
        let value = self.fetch()?;
        (OpCode::CPX, AddressingMode::ZeroPage(value), 3)
      }
      0xEC => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CPX, AddressingMode::Absolute(value), 4)
      }
      // CPY
      0xC0 => {
        let value = self.fetch()?;
        (OpCode::CPY, AddressingMode::Immediate(value), 2)
      }
      0xC4 => {
        let value = self.fetch()?;
        (OpCode::CPY, AddressingMode::ZeroPage(value), 3)
      }
      0xCC => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::CPY, AddressingMode::Absolute(value), 4)
      }
      // DEC
      0xC6 => {
        let value = self.fetch()?;
        (OpCode::DEC, AddressingMode::ZeroPage(value), 5)
      }
      0xD6 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::DEC, AddressingMode::ZeroPageX(value), 6)
      }
      0xCE => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::DEC, AddressingMode::Absolute(value), 6)
      }
      0xDE => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::DEC, AddressingMode::AbsoluteX(value), 7)
      }
      // EOR
      0x49 => {
        let value = self.fetch()?;
        (OpCode::EOR, AddressingMode::Immediate(value), 2)
      }
      0x45 => {
        let value = self.fetch()?;
        (OpCode::EOR, AddressingMode::ZeroPage(value), 3)
      }
      0x55 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::EOR, AddressingMode::ZeroPageX(value), 4)
      }
      0x4D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::EOR, AddressingMode::Absolute(value), 4)
      }
      0x5D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::EOR, AddressingMode::AbsoluteX(value), 4)
      }
      0x59 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::EOR, AddressingMode::AbsoluteY(value), 4)
      }
      0x41 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::EOR, AddressingMode::IndirectX(value), 6)
      }
      0x51 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::EOR, AddressingMode::IndirectY(value), 5)
      }
      // FLAGS
      0x18 => (OpCode::CLC, AddressingMode::Implied, 2),
      0x38 => (OpCode::SEC, AddressingMode::Implied, 2),
      0x58 => (OpCode::CLI, AddressingMode::Implied, 2),
      0x78 => (OpCode::SEI, AddressingMode::Implied, 2),
      0xB8 => (OpCode::CLV, AddressingMode::Implied, 2),
      0xD8 => (OpCode::CLD, AddressingMode::Implied, 2),
      0xF8 => (OpCode::SED, AddressingMode::Implied, 2),
      // INC
      0xE6 => {
        let value = self.fetch()?;
        (OpCode::INC, AddressingMode::ZeroPage(value), 5)
      }
      0xF6 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::INC, AddressingMode::ZeroPageX(value), 6)
      }
      0xEE => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::INC, AddressingMode::Absolute(value), 6)
      }
      0xFE => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::INC, AddressingMode::AbsoluteX(value), 7)
      }
      // JMP
      0x4C => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::JMP, AddressingMode::Absolute(value), 3)
      }
      0x6C => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::JMP, AddressingMode::Indirect(value), 5)
      }
      // JSR
      0x20 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::JSR, AddressingMode::Absolute(value), 6)
      }
      // LDA
      0xA9 => {
        let value = self.fetch()?;
        (OpCode::LDA, AddressingMode::Immediate(value), 2)
      }
      0xA5 => {
        let value = self.fetch()?;
        (OpCode::LDA, AddressingMode::ZeroPage(value), 3)
      }
      0xB5 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDA, AddressingMode::ZeroPageX(value), 4)
      }
      0xAD => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDA, AddressingMode::Absolute(value), 4)
      }
      0xBD => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDA, AddressingMode::AbsoluteX(value), 4)
      }
      0xB9 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDA, AddressingMode::AbsoluteY(value), 4)
      }
      0xA1 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDA, AddressingMode::IndirectX(value), 6)
      }
      0xB1 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDA, AddressingMode::IndirectY(value), 5)
      }
      // LDX
      0xA2 => {
        let value = self.fetch()?;
        (OpCode::LDX, AddressingMode::Immediate(value), 2)
      }
      0xA6 => {
        let value = self.fetch()?;
        (OpCode::LDX, AddressingMode::ZeroPage(value), 3)
      }
      0xB6 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDX, AddressingMode::ZeroPageY(value), 4)
      }
      0xAE => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDX, AddressingMode::Absolute(value), 4)
      }
      0xBE => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDX, AddressingMode::AbsoluteY(value), 4)
      }
      // LDY
      0xA0 => {
        let value = self.fetch()?;
        (OpCode::LDY, AddressingMode::Immediate(value), 2)
      }
      0xA4 => {
        let value = self.fetch()?;
        (OpCode::LDY, AddressingMode::ZeroPage(value), 3)
      }
      0xB4 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDY, AddressingMode::ZeroPageX(value), 4)
      }
      0xAC => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDY, AddressingMode::Absolute(value), 4)
      }
      0xBC => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LDY, AddressingMode::AbsoluteX(value), 4)
      }
      // LSR
      0x4A => (OpCode::LSR, AddressingMode::Accumulator, 2),
      0x46 => {
        let value = self.fetch()?;
        (OpCode::LSR, AddressingMode::ZeroPage(value), 5)
      }
      0x56 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LSR, AddressingMode::ZeroPageX(value), 6)
      }
      0x4E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LSR, AddressingMode::Absolute(value), 6)
      }
      0x5E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::LSR, AddressingMode::AbsoluteX(value), 7)
      }
      // NOP
      0xEA => (OpCode::NOP, AddressingMode::Implied, 2),
      // ORA
      0x09 => {
        let value = self.fetch()?;
        (OpCode::ORA, AddressingMode::Immediate(value), 2)
      }
      0x05 => {
        let value = self.fetch()?;
        (OpCode::ORA, AddressingMode::ZeroPage(value), 3)
      }
      0x15 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ORA, AddressingMode::ZeroPageX(value), 4)
      }
      0x0D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ORA, AddressingMode::Absolute(value), 4)
      }
      0x1D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ORA, AddressingMode::AbsoluteX(value), 4)
      }
      0x19 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ORA, AddressingMode::AbsoluteY(value), 4)
      }
      0x01 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ORA, AddressingMode::IndirectX(value), 6)
      }
      0x11 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ORA, AddressingMode::IndirectY(value), 5)
      }
      // REGISTER
      0xAA => (OpCode::TAX, AddressingMode::Implied, 2),
      0x8A => (OpCode::TXA, AddressingMode::Implied, 2),
      0xCA => (OpCode::DEX, AddressingMode::Implied, 2),
      0xE8 => (OpCode::INX, AddressingMode::Implied, 2),
      0xA8 => (OpCode::TAY, AddressingMode::Implied, 2),
      0x98 => (OpCode::TYA, AddressingMode::Implied, 2),
      0x88 => (OpCode::DEY, AddressingMode::Implied, 2),
      0xC8 => (OpCode::INY, AddressingMode::Implied, 2),
      // ROL
      0x2A => (OpCode::ROL, AddressingMode::Accumulator, 2),
      0x26 => {
        let value = self.fetch()?;
        (OpCode::ROL, AddressingMode::ZeroPage(value), 5)
      }
      0x36 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ROL, AddressingMode::ZeroPageX(value), 6)
      }
      0x2E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ROL, AddressingMode::Absolute(value), 6)
      }
      0x3E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ROL, AddressingMode::AbsoluteX(value), 7)
      }
      // ROR
      0x6A => (OpCode::ROR, AddressingMode::Accumulator, 2),
      0x66 => {
        let value = self.fetch()?;
        (OpCode::ROR, AddressingMode::ZeroPage(value), 5)
      }
      0x76 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ROR, AddressingMode::ZeroPageX(value), 6)
      }
      0x6E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ROR, AddressingMode::Absolute(value), 6)
      }
      0x7E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::ROR, AddressingMode::AbsoluteX(value), 7)
      }
      // RTI
      0x40 => (OpCode::RTI, AddressingMode::Implied, 6),
      // RTS
      0x60 => (OpCode::RTS, AddressingMode::Implied, 6),
      // SBC
      0xE9 => {
        let value = self.fetch()?;
        (OpCode::SBC, AddressingMode::Immediate(value), 2)
      }
      0xE5 => {
        let value = self.fetch()?;
        (OpCode::SBC, AddressingMode::ZeroPage(value), 3)
      }
      0xF5 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::SBC, AddressingMode::ZeroPageX(value), 4)
      }
      0xED => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::SBC, AddressingMode::Absolute(value), 4)
      }
      0xFD => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::SBC, AddressingMode::AbsoluteX(value), 4)
      }
      0xF9 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::SBC, AddressingMode::AbsoluteY(value), 4)
      }
      0xE1 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::SBC, AddressingMode::IndirectX(value), 6)
      }
      0xF1 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::SBC, AddressingMode::IndirectY(value), 5)
      }
      // STA
      0x85 => {
        let value = self.fetch()?;
        (OpCode::STA, AddressingMode::ZeroPage(value), 3)
      }
      0x95 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STA, AddressingMode::ZeroPageX(value), 4)
      }
      0x8D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STA, AddressingMode::Absolute(value), 4)
      }
      0x9D => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STA, AddressingMode::AbsoluteX(value), 5)
      }
      0x99 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STA, AddressingMode::AbsoluteY(value), 5)
      }
      0x81 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STA, AddressingMode::IndirectX(value), 6)
      }
      0x91 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STA, AddressingMode::IndirectY(value), 6)
      }
      // STACK
      0x9A => (OpCode::TXS, AddressingMode::Implied, 2),
      0xBA => (OpCode::TSX, AddressingMode::Implied, 2),
      0x48 => (OpCode::PHA, AddressingMode::Implied, 3),
      0x68 => (OpCode::PLA, AddressingMode::Implied, 4),
      0x08 => (OpCode::PHP, AddressingMode::Implied, 3),
      0x28 => (OpCode::PLP, AddressingMode::Implied, 4),
      // STX
      0x86 => {
        let value = self.fetch()?;
        (OpCode::STX, AddressingMode::ZeroPage(value), 3)
      }
      0x96 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STX, AddressingMode::ZeroPageY(value), 4)
      }
      0x8E => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STX, AddressingMode::Absolute(value), 4)
      }
      // STY
      0x84 => {
        let value = self.fetch()?;
        (OpCode::STY, AddressingMode::ZeroPage(value), 3)
      }
      0x94 => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STY, AddressingMode::ZeroPageX(value), 4)
      }
      0x8C => {
        let first = self.fetch()?;
        let second = self.fetch()?;
        let value = u16::from_le_bytes([first, second]);
        (OpCode::STY, AddressingMode::Absolute(value), 4)
      }
      _ => Err(CpuError::InvalidOpCode(opcode))?,
    };

    Ok(Instruction { opcode, mode, cycles })
  }
}
