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
      0x69 => (OpCode::ADC, AddressingMode::immediate(self), 2),
      0x65 => (OpCode::ADC, AddressingMode::zero_page(self), 3),
      0x75 => (OpCode::ADC, AddressingMode::zero_page_x(self), 4),
      0x6D => (OpCode::ADC, AddressingMode::absolute(self), 4),
      0x7D => (OpCode::ADC, AddressingMode::absolute_x(self), 4),
      0x79 => (OpCode::ADC, AddressingMode::absolute_y(self), 4),
      0x61 => (OpCode::ADC, AddressingMode::indirect_x(self), 6),
      0x71 => (OpCode::ADC, AddressingMode::indirect_y(self), 5),
      // AND
      0x29 => (OpCode::AND, AddressingMode::immediate(self), 2),
      0x25 => (OpCode::AND, AddressingMode::zero_page(self), 3),
      0x35 => (OpCode::AND, AddressingMode::zero_page_x(self), 4),
      0x2D => (OpCode::AND, AddressingMode::absolute(self), 4),
      0x3D => (OpCode::AND, AddressingMode::absolute_x(self), 4),
      0x39 => (OpCode::AND, AddressingMode::absolute_y(self), 4),
      0x21 => (OpCode::AND, AddressingMode::indirect_x(self), 6),
      0x31 => (OpCode::AND, AddressingMode::indirect_y(self), 5),
      // ASL
      0x0A => (OpCode::ASL, AddressingMode::accumulator(self), 2),
      0x06 => (OpCode::ASL, AddressingMode::zero_page(self), 5),
      0x16 => (OpCode::ASL, AddressingMode::zero_page_x(self), 6),
      0x0E => (OpCode::ASL, AddressingMode::absolute(self), 6),
      0x1E => (OpCode::ASL, AddressingMode::absolute_x(self), 7),
      // BIT
      0x24 => (OpCode::BIT, AddressingMode::zero_page(self), 3),
      0x2C => (OpCode::BIT, AddressingMode::absolute(self), 4),
      // BRANCH
      0x10 => (OpCode::BPL, AddressingMode::relative(self), 2),
      0x30 => (OpCode::BMI, AddressingMode::relative(self), 2),
      0x50 => (OpCode::BVC, AddressingMode::relative(self), 2),
      0x70 => (OpCode::BVS, AddressingMode::relative(self), 2),
      0x90 => (OpCode::BCC, AddressingMode::relative(self), 2),
      0xB0 => (OpCode::BCS, AddressingMode::relative(self), 2),
      0xD0 => (OpCode::BNE, AddressingMode::relative(self), 2),
      0xF0 => (OpCode::BEQ, AddressingMode::relative(self), 2),
      // BREAK
      0x00 => (OpCode::BRK, AddressingMode::implied(self), 7),
      // CMP
      0xC9 => (OpCode::CMP, AddressingMode::immediate(self), 2),
      0xC5 => (OpCode::CMP, AddressingMode::zero_page(self), 3),
      0xD5 => (OpCode::CMP, AddressingMode::zero_page_x(self), 4),
      0xCD => (OpCode::CMP, AddressingMode::absolute(self), 4),
      0xDD => (OpCode::CMP, AddressingMode::absolute_x(self), 4),
      0xD9 => (OpCode::CMP, AddressingMode::absolute_y(self), 4),
      0xC1 => (OpCode::CMP, AddressingMode::indirect_x(self), 6),
      0xD1 => (OpCode::CMP, AddressingMode::indirect_y(self), 5),
      // CPX
      0xE0 => (OpCode::CPX, AddressingMode::immediate(self), 2),
      0xE4 => (OpCode::CPX, AddressingMode::zero_page(self), 3),
      0xEC => (OpCode::CPX, AddressingMode::absolute(self), 4),
      // CPY
      0xC0 => (OpCode::CPY, AddressingMode::immediate(self), 2),
      0xC4 => (OpCode::CPY, AddressingMode::zero_page(self), 3),
      0xCC => (OpCode::CPY, AddressingMode::absolute(self), 4),
      // DEC
      0xC6 => (OpCode::DEC, AddressingMode::zero_page(self), 5),
      0xD6 => (OpCode::DEC, AddressingMode::zero_page_x(self), 6),
      0xCE => (OpCode::DEC, AddressingMode::absolute(self), 6),
      0xDE => (OpCode::DEC, AddressingMode::absolute_x(self), 7),
      // EOR
      0x49 => (OpCode::EOR, AddressingMode::immediate(self), 2),
      0x45 => (OpCode::EOR, AddressingMode::zero_page(self), 3),
      0x55 => (OpCode::EOR, AddressingMode::zero_page_x(self), 4),
      0x4D => (OpCode::EOR, AddressingMode::absolute(self), 4),
      0x5D => (OpCode::EOR, AddressingMode::absolute_x(self), 4),
      0x59 => (OpCode::EOR, AddressingMode::absolute_y(self), 4),
      0x41 => (OpCode::EOR, AddressingMode::indirect_x(self), 6),
      0x51 => (OpCode::EOR, AddressingMode::indirect_y(self), 5),
      // FLAGS
      0x18 => (OpCode::CLC, AddressingMode::implied(self), 2),
      0x38 => (OpCode::SEC, AddressingMode::implied(self), 2),
      0x58 => (OpCode::CLI, AddressingMode::implied(self), 2),
      0x78 => (OpCode::SEI, AddressingMode::implied(self), 2),
      0xB8 => (OpCode::CLV, AddressingMode::implied(self), 2),
      0xD8 => (OpCode::CLD, AddressingMode::implied(self), 2),
      0xF8 => (OpCode::SED, AddressingMode::implied(self), 2),
      // INC
      0xE6 => (OpCode::INC, AddressingMode::zero_page(self), 5),
      0xF6 => (OpCode::INC, AddressingMode::zero_page_x(self), 6),
      0xEE => (OpCode::INC, AddressingMode::absolute(self), 6),
      0xFE => (OpCode::INC, AddressingMode::absolute_x(self), 7),
      // JMP
      0x4C => (OpCode::JMP, AddressingMode::absolute(self), 3),
      0x6C => (OpCode::JMP, AddressingMode::indirect(self), 5),
      // JSR
      0x20 => (OpCode::JSR, AddressingMode::absolute(self), 6),
      // LDA
      0xA9 => (OpCode::LDA, AddressingMode::immediate(self), 2),
      0xA5 => (OpCode::LDA, AddressingMode::zero_page(self), 3),
      0xB5 => (OpCode::LDA, AddressingMode::zero_page_x(self), 4),
      0xAD => (OpCode::LDA, AddressingMode::absolute(self), 4),
      0xBD => (OpCode::LDA, AddressingMode::absolute_x(self), 4),
      0xB9 => (OpCode::LDA, AddressingMode::absolute_y(self), 4),
      0xA1 => (OpCode::LDA, AddressingMode::indirect_x(self), 6),
      0xB1 => (OpCode::LDA, AddressingMode::indirect_y(self), 5),
      // LDX
      0xA2 => (OpCode::LDX, AddressingMode::immediate(self), 2),
      0xA6 => (OpCode::LDX, AddressingMode::zero_page(self), 3),
      0xB6 => (OpCode::LDX, AddressingMode::zero_page_y(self), 4),
      0xAE => (OpCode::LDX, AddressingMode::absolute(self), 4),
      0xBE => (OpCode::LDX, AddressingMode::absolute_y(self), 4),
      // LDY
      0xA0 => (OpCode::LDY, AddressingMode::immediate(self), 2),
      0xA4 => (OpCode::LDY, AddressingMode::zero_page(self), 3),
      0xB4 => (OpCode::LDY, AddressingMode::zero_page_x(self), 4),
      0xAC => (OpCode::LDY, AddressingMode::absolute(self), 4),
      0xBC => (OpCode::LDY, AddressingMode::absolute_x(self), 4),
      // LSR
      0x4A => (OpCode::LSR, AddressingMode::accumulator(self), 2),
      0x46 => (OpCode::LSR, AddressingMode::zero_page(self), 5),
      0x56 => (OpCode::LSR, AddressingMode::zero_page_x(self), 6),
      0x4E => (OpCode::LSR, AddressingMode::absolute(self), 6),
      0x5E => (OpCode::LSR, AddressingMode::absolute_x(self), 7),
      // NOP
      0xEA => (OpCode::NOP, AddressingMode::implied(self), 2),
      // ORA
      0x09 => (OpCode::ORA, AddressingMode::immediate(self), 2),
      0x05 => (OpCode::ORA, AddressingMode::zero_page(self), 3),
      0x15 => (OpCode::ORA, AddressingMode::zero_page_x(self), 4),
      0x0D => (OpCode::ORA, AddressingMode::absolute(self), 4),
      0x1D => (OpCode::ORA, AddressingMode::absolute_x(self), 4),
      0x19 => (OpCode::ORA, AddressingMode::absolute_y(self), 4),
      0x01 => (OpCode::ORA, AddressingMode::indirect_x(self), 6),
      0x11 => (OpCode::ORA, AddressingMode::indirect_y(self), 5),
      // REGISTER
      0xAA => (OpCode::TAX, AddressingMode::implied(self), 2),
      0x8A => (OpCode::TXA, AddressingMode::implied(self), 2),
      0xCA => (OpCode::DEX, AddressingMode::implied(self), 2),
      0xE8 => (OpCode::INX, AddressingMode::implied(self), 2),
      0xA8 => (OpCode::TAY, AddressingMode::implied(self), 2),
      0x98 => (OpCode::TYA, AddressingMode::implied(self), 2),
      0x88 => (OpCode::DEY, AddressingMode::implied(self), 2),
      0xC8 => (OpCode::INY, AddressingMode::implied(self), 2),
      // ROL
      0x2A => (OpCode::ROL, AddressingMode::accumulator(self), 2),
      0x26 => (OpCode::ROL, AddressingMode::zero_page(self), 5),
      0x36 => (OpCode::ROL, AddressingMode::zero_page_x(self), 6),
      0x2E => (OpCode::ROL, AddressingMode::absolute(self), 6),
      0x3E => (OpCode::ROL, AddressingMode::absolute_x(self), 7),
      // ROR
      0x6A => (OpCode::ROR, AddressingMode::accumulator(self), 2),
      0x66 => (OpCode::ROR, AddressingMode::zero_page(self), 5),
      0x76 => (OpCode::ROR, AddressingMode::zero_page_x(self), 6),
      0x6E => (OpCode::ROR, AddressingMode::absolute(self), 6),
      0x7E => (OpCode::ROR, AddressingMode::absolute_x(self), 7),
      // RTI
      0x40 => (OpCode::RTI, AddressingMode::implied(self), 6),
      // RTS
      0x60 => (OpCode::RTS, AddressingMode::implied(self), 6),
      // SBC
      0xE9 => (OpCode::SBC, AddressingMode::immediate(self), 2),
      0xE5 => (OpCode::SBC, AddressingMode::zero_page(self), 3),
      0xF5 => (OpCode::SBC, AddressingMode::zero_page_x(self), 4),
      0xED => (OpCode::SBC, AddressingMode::absolute(self), 4),
      0xFD => (OpCode::SBC, AddressingMode::absolute_x(self), 4),
      0xF9 => (OpCode::SBC, AddressingMode::absolute_y(self), 4),
      0xE1 => (OpCode::SBC, AddressingMode::indirect_x(self), 6),
      0xF1 => (OpCode::SBC, AddressingMode::indirect_y(self), 5),
      // STA
      0x85 => (OpCode::STA, AddressingMode::zero_page(self), 3),
      0x95 => (OpCode::STA, AddressingMode::zero_page_x(self), 4),
      0x8D => (OpCode::STA, AddressingMode::absolute(self), 4),
      0x9D => (OpCode::STA, AddressingMode::absolute_x(self), 5),
      0x99 => (OpCode::STA, AddressingMode::absolute_y(self), 5),
      0x81 => (OpCode::STA, AddressingMode::indirect_x(self), 6),
      0x91 => (OpCode::STA, AddressingMode::indirect_y(self), 6),
      // STACK
      0x9A => (OpCode::TXS, AddressingMode::implied(self), 2),
      0xBA => (OpCode::TSX, AddressingMode::implied(self), 2),
      0x48 => (OpCode::PHA, AddressingMode::implied(self), 3),
      0x68 => (OpCode::PLA, AddressingMode::implied(self), 4),
      0x08 => (OpCode::PHP, AddressingMode::implied(self), 3),
      0x28 => (OpCode::PLP, AddressingMode::implied(self), 4),
      // STX
      0x86 => (OpCode::STX, AddressingMode::zero_page(self), 3),
      0x96 => (OpCode::STX, AddressingMode::zero_page_y(self), 4),
      0x8E => (OpCode::STX, AddressingMode::absolute(self), 4),
      // STY
      0x84 => (OpCode::STY, AddressingMode::zero_page(self), 3),
      0x94 => (OpCode::STY, AddressingMode::zero_page_x(self), 4),
      0x8C => (OpCode::STY, AddressingMode::absolute(self), 4),
      _ => Err(CpuError::InvalidOpCode(opcode))?,
    };

    let mode = mode?;

    Ok(Instruction { opcode, mode, cycles })
  }
}
