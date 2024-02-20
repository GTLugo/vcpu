use crate::{
  core::{
    cpu::{Cpu, StatusFlags},
    instruction::{AddressingMode, Instruction, OpCode},
  },
  error::CpuError,
};

impl Cpu {
  /// Returns how many cycles needed for completion of instruction
  pub(crate) fn execute(&mut self, instruction: Instruction) -> Result<u8, CpuError> {
    let Instruction {
      opcode,
      mode,
      mut cycles,
    } = instruction;

    let (data, address) = self.interpret_payload(&mode, &mut cycles);

    match opcode {
      OpCode::ADC => {
        let input_carry = self.is_flag(StatusFlags::Carry);
        let (result, carry) = self.a.carrying_add(data, input_carry);
        let overflow = input_carry != carry; // https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.set_flag(StatusFlags::Overflow, overflow);
        self.set_flag(StatusFlags::Carry, carry);
        self.set_flag(StatusFlags::Zero, result == 0x00);
        self.set_flag(StatusFlags::Negative, (result & 0x80) == 0x80);
        self.a = result;
      }
      OpCode::AND => {
        self.a &= data;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::ASL => {
        let shifted = (data as u16) << 1;
        self.set_flag(StatusFlags::Carry, (shifted & 0xFF00) > 0);
        self.set_flag(StatusFlags::Zero, (shifted & 0xFF00) > 0);
        self.set_flag(StatusFlags::Negative, (shifted & 0x0080) == 0x0080);
        if let AddressingMode::Implied = mode {
          self.a = shifted as u8;
        } else {
          self.write(address, shifted as u8);
        }
      }
      OpCode::BCC => {
        if !self.is_flag(StatusFlags::Carry) {
          cycles += 1;
          let (addr, overflow) = self.prog_counter.overflowing_add(address);
          if overflow {
            cycles += 1;
          }
          self.prog_counter = addr;
        }
      }
      OpCode::BCS => {
        if self.is_flag(StatusFlags::Carry) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::BEQ => {
        if self.is_flag(StatusFlags::Zero) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::BIT => {
        let temp = self.a & data;
        self.set_flag(StatusFlags::Overflow, temp == 0x00);
        self.set_flag(StatusFlags::Zero, data & (1 << 7) == (1 << 7));
        self.set_flag(StatusFlags::Negative, data & (1 << 6) == (1 << 6));
      }
      OpCode::BMI => {
        if self.is_flag(StatusFlags::Negative) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::BNE => {
        if !self.is_flag(StatusFlags::Zero) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::BPL => {
        if !self.is_flag(StatusFlags::Negative) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::BRK => {
        self.set_flag(StatusFlags::Interrupt, true);
        let [lo, hi] = self.prog_counter.to_le_bytes();
        self.push(hi);
        self.push(lo);
        self.set_flag(StatusFlags::Break, true);
        self.push(self.status.bits());
        self.set_flag(StatusFlags::Break, false);
        
        let lo = self.read(Self::INTERRUPT_ADDRESS);
        let hi = self.read(Self::INTERRUPT_ADDRESS + 1);
        self.prog_counter = u16::from_le_bytes([lo, hi]);
      },
      OpCode::BVC => {
        if !self.is_flag(StatusFlags::Overflow) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::BVS => {
        if self.is_flag(StatusFlags::Overflow) {
          self.branch(address, &mut cycles);
        }
      }
      OpCode::CLC => {
        self.set_flag(StatusFlags::Carry, false);
      }
      OpCode::CLD => {
        self.set_flag(StatusFlags::Decimal, false);
      }
      OpCode::CLI => {
        self.set_flag(StatusFlags::Interrupt, false);
      }
      OpCode::CLV => {
        self.set_flag(StatusFlags::Overflow, false);
      }
      OpCode::CMP => todo!(),
      OpCode::CPX => todo!(),
      OpCode::CPY => todo!(),
      OpCode::DEC => todo!(),
      OpCode::DEX => todo!(),
      OpCode::DEY => todo!(),
      OpCode::EOR => todo!(),
      OpCode::INC => todo!(),
      OpCode::INX => todo!(),
      OpCode::INY => todo!(),
      OpCode::JMP => todo!(),
      OpCode::JSR => todo!(),
      OpCode::LDA => todo!(),
      OpCode::LDX => todo!(),
      OpCode::LDY => todo!(),
      OpCode::LSR => todo!(),
      OpCode::NOP => todo!(),
      OpCode::ORA => todo!(),
      OpCode::PHA => {
        self.push(self.a);
      }
      OpCode::PHP => todo!(),
      OpCode::PLA => {
        self.a = self.pop();
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::PLP => todo!(),
      OpCode::ROL => todo!(),
      OpCode::ROR => todo!(),
      OpCode::RTI => {
        self.status = self.pop().try_into().map_err(|e| CpuError::Other(format!("{e}")))?;
        self.status.toggle(StatusFlags::Break);
        self.status.toggle(StatusFlags::Unused);

        let lo = self.pop();
        let hi = self.pop();
        self.prog_counter = u16::from_le_bytes([lo, hi]);
      }
      OpCode::RTS => todo!(),
      OpCode::SBC => {
        let input_data = !data;
        let input_carry = self.is_flag(StatusFlags::Carry);
        let (result, carry) = self.a.carrying_add(input_data, input_carry);
        let overflow = input_carry != carry; // https://www.righto.com/2012/12/the-6502-overflow-flag-explained.html
        self.set_flag(StatusFlags::Overflow, overflow);
        self.set_flag(StatusFlags::Carry, carry);
        self.set_flag(StatusFlags::Zero, result == 0x00);
        self.set_flag(StatusFlags::Negative, (result & 0x80) == 0x80);
        self.a = result;
      }
      OpCode::SEC => {
        self.set_flag(StatusFlags::Carry, true);
      }
      OpCode::SED => {
        self.set_flag(StatusFlags::Decimal, true);
      }
      OpCode::SEI => {
        self.set_flag(StatusFlags::Interrupt, true);
      }
      OpCode::STA => todo!(),
      OpCode::STX => todo!(),
      OpCode::STY => todo!(),
      OpCode::TAX => todo!(),
      OpCode::TAY => todo!(),
      OpCode::TSX => todo!(),
      OpCode::TXA => todo!(),
      OpCode::TXS => todo!(),
      OpCode::TYA => todo!(),
    }

    Ok(cycles)
  }

  fn interpret_payload(&mut self, mode: &AddressingMode, cycles: &mut u8) -> (u8, u16) {
    let address = match mode {
      AddressingMode::Accumulator => self.a as u16,
      AddressingMode::Implied => {
        return (self.a, 0);
      }
      AddressingMode::Immediate(value) => {
        return (*value, self.prog_counter - 1);
      },
      AddressingMode::Relative(addr) => {
        let mut address = *addr as u16;
        if (address & 0x80) == 0x80 {
          // 7th bit is 1, so this is supposed to be signed. set the top bits to all 1s to
          // enable 2s compliment arithmetic
          address |= 0xFF00;
        }
        address
      }
      AddressingMode::ZeroPage(addr) => *addr as u16 & 0x00FF,
      AddressingMode::ZeroPageX(addr) => (addr + self.x as u16) & 0x00FF,
      AddressingMode::ZeroPageY(addr) => (addr + self.y as u16) & 0x00FF,
      AddressingMode::Absolute(addr) => *addr,
      AddressingMode::AbsoluteX(addr) => {
        let (addr, overflow) = addr.overflowing_add(self.x as u16);
        if overflow {
          *cycles += 1;
        }
        addr
      }
      AddressingMode::AbsoluteY(addr) => {
        let (addr, overflow) = addr.overflowing_add(self.y as u16);
        if overflow {
          *cycles += 1;
        }
        addr
      }
      AddressingMode::Indirect(ptr) => {
        if (ptr & 0x00FF) == 0x00FF {
          // simulate hardware bug
          let lo = self.read(*ptr);
          let hi = self.read(ptr & 0xFF00);
          u16::from_le_bytes([lo, hi])
        } else {
          // behave normally
          let lo = self.read(*ptr);
          let hi = self.read(ptr + 1);
          u16::from_le_bytes([lo, hi])
        }
      }
      AddressingMode::IndirectX(ptr) => {
        let lo = self.read((ptr + self.x as u16) & 0x00FF);
        let hi = self.read((ptr + self.x as u16 + 1) & 0x00FF);
        u16::from_le_bytes([lo, hi])
      }
      AddressingMode::IndirectY(ptr) => {
        let lo = self.read((ptr) & 0x00FF);
        let hi = self.read((ptr + 1) & 0x00FF);
        let (addr, overflow) = u16::from_le_bytes([lo, hi]).overflowing_add(self.y as u16);
        if overflow {
          *cycles += 1;
        }
        addr
      }
    };

    let data = self.read(address);

    (data, address)
  }

  fn branch(&mut self, payload: u16, cycles: &mut u8) {
    *cycles += 1;
    let (addr, overflow) = self.prog_counter.overflowing_add(payload);
    if overflow {
      *cycles += 1;
    }
    self.prog_counter = addr;
  }
}
