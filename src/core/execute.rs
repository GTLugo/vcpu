use crate::core::cpu::{Cpu, StatusFlags};
use crate::core::instruction::{AddressingMode, Instruction, OpCode};
use crate::error::CpuError;

impl Cpu {
  /// Returns how many cycles needed for completion of instruction
  pub(crate) fn execute(&mut self, instruction: Instruction) -> Result<u8, CpuError> {
    let Instruction { opcode, mode, mut cycles } = instruction;

    let data: u8 = match mode {
      AddressingMode::Accumulator => self.a,
      AddressingMode::Implied => 0,
      AddressingMode::Immediate(value) => value,
      AddressingMode::Relative(addr) => {
        let mut relative = addr as u16;
        if (relative & 0x80) == 0x80 {
          // 7th bit is 1, so this is supposed to be signed. set the top bits to all 1s to enable
          // 2s compliment arithmetic
          relative |= 0xFF00;
        }
        self.read(relative)
      }
      AddressingMode::ZeroPage(addr) => {
        self.read(addr as u16 & 0x00FF)
      }
      AddressingMode::ZeroPageX(addr) => {
        self.read((addr + self.x as u16) & 0x00FF)
      }
      AddressingMode::ZeroPageY(addr) => {
        self.read((addr + self.y as u16) & 0x00FF)
      }
      AddressingMode::Absolute(addr) => {
        self.read(addr)
      }
      AddressingMode::AbsoluteX(addr) => {
        let (addr, overflow) = addr.overflowing_add(self.x as u16);
        if overflow {
          cycles += 1;
        }
        self.read(addr)
      }
      AddressingMode::AbsoluteY(addr) => {
        let (addr, overflow) = addr.overflowing_add(self.y as u16);
        if overflow {
          cycles += 1;
        }
        self.read(addr)
      }
      AddressingMode::Indirect(ptr) => {
        if (ptr & 0x00FF) == 0x00FF {
          // simulate hardware bug
          let lo = self.read(ptr);
          let hi = self.read(ptr & 0xFF00);
          self.read(u16::from_le_bytes([lo, hi]))
        } else {
          // behave normally
          let lo = self.read(ptr);
          let hi = self.read(ptr + 1);
          self.read(u16::from_le_bytes([lo, hi]))
        }
      }
      AddressingMode::IndirectX(ptr) => {
        let lo = self.read((ptr + self.x as u16) & 0x00FF);
        let hi = self.read((ptr + self.x as u16 + 1) & 0x00FF);
        self.read(u16::from_le_bytes([lo, hi]))
      }
      AddressingMode::IndirectY(ptr) => {
        let lo = self.read((ptr) & 0x00FF);
        let hi = self.read((ptr + 1) & 0x00FF);
        let (addr, overflow) = u16::from_le_bytes([lo, hi]).overflowing_add(self.y as u16);
        if overflow {
          cycles += 1;
        }
        self.read(addr)
      }
    };

    match opcode {
      OpCode::ADC => {}
      OpCode::AND => {
        self.a &= data;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::ASL => {}
      OpCode::BCC => {}
      OpCode::BCS => {}
      OpCode::BEQ => {}
      OpCode::BIT => {}
      OpCode::BMI => {}
      OpCode::BNE => {}
      OpCode::BPL => {}
      OpCode::BRK => {}
      OpCode::BVC => {}
      OpCode::BVS => {}
      OpCode::CLC => {}
      OpCode::CLD => {}
      OpCode::CLI => {}
      OpCode::CLV => {}
      OpCode::CMP => {}
      OpCode::CPX => {}
      OpCode::CPY => {}
      OpCode::DEC => {}
      OpCode::DEX => {}
      OpCode::DEY => {}
      OpCode::EOR => {}
      OpCode::INC => {}
      OpCode::INX => {}
      OpCode::INY => {}
      OpCode::JMP => {}
      OpCode::JSR => {}
      OpCode::LDA => {}
      OpCode::LDX => {}
      OpCode::LDY => {}
      OpCode::LSR => {}
      OpCode::NOP => {}
      OpCode::ORA => {}
      OpCode::PHA => {}
      OpCode::PHP => {}
      OpCode::PLA => {}
      OpCode::PLP => {}
      OpCode::ROL => {}
      OpCode::ROR => {}
      OpCode::RTI => {}
      OpCode::RTS => {}
      OpCode::SBC => {}
      OpCode::SEC => {}
      OpCode::SED => {}
      OpCode::SEI => {}
      OpCode::STA => {}
      OpCode::STX => {}
      OpCode::STY => {}
      OpCode::TAX => {}
      OpCode::TAY => {}
      OpCode::TSX => {}
      OpCode::TXA => {}
      OpCode::TXS => {}
      OpCode::TYA => {}
    }

    Ok(cycles)
  }
}