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
        if let AddressingMode::Implied | AddressingMode::Accumulator = mode {
          self.a = shifted as u8;
        } else {
          self.write(address, shifted as u8);
        }
      }
      OpCode::BCC => {
        if !self.is_flag(StatusFlags::Carry) {
          cycles += 1;
          let (addr, overflow) = self.program_counter.overflowing_add(address);
          if overflow {
            cycles += 1;
          }
          self.program_counter = addr;
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
        let [lo, hi] = self.program_counter.to_le_bytes();
        self.push(hi);
        self.push(lo);
        self.set_flag(StatusFlags::Break, true);
        self.push(self.status.bits());
        self.set_flag(StatusFlags::Break, false);

        let lo = self.read(Self::INTERRUPT_ADDRESS);
        let hi = self.read(Self::INTERRUPT_ADDRESS + 1);
        self.program_counter = u16::from_le_bytes([lo, hi]);
      }
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
      OpCode::CMP => {
        let temp = self.a - data;
        self.set_flag(StatusFlags::Carry, self.a >= data);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
      }
      OpCode::CPX => {
        let temp = self.x - data;
        self.set_flag(StatusFlags::Carry, self.a >= data);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
      }
      OpCode::CPY => {
        let temp = self.y - data;
        self.set_flag(StatusFlags::Carry, self.a >= data);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
      }
      OpCode::DEC => {
        let temp = data - 1;
        self.write(address, temp);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
      }
      OpCode::DEX => {
        self.x -= 1;
        self.set_flag(StatusFlags::Zero, self.x == 0x00);
        self.set_flag(StatusFlags::Negative, (self.x & 0x80) == 0x80);
      }
      OpCode::DEY => {
        self.y -= 1;
        self.set_flag(StatusFlags::Zero, self.y == 0x00);
        self.set_flag(StatusFlags::Negative, (self.y & 0x80) == 0x80);
      }
      OpCode::EOR => {
        self.a ^= data;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::INC => {
        let temp = data + 1;
        self.write(address, temp);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
      }
      OpCode::INX => {
        self.x += 1;
        self.set_flag(StatusFlags::Zero, self.x == 0x00);
        self.set_flag(StatusFlags::Negative, (self.x & 0x80) == 0x80);
      }
      OpCode::INY => {
        self.y += 1;
        self.set_flag(StatusFlags::Zero, self.y == 0x00);
        self.set_flag(StatusFlags::Negative, (self.y & 0x80) == 0x80);
      }
      OpCode::JMP => {
        self.program_counter = address;
      }
      OpCode::JSR => {
        // self.program_counter -= 1;
        let [lo, hi] = self.program_counter.to_le_bytes();
        self.push(hi);
        self.push(lo);
        self.program_counter = address;
      }
      OpCode::LDA => {
        self.a = data;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::LDX => {
        self.x = data;
        self.set_flag(StatusFlags::Zero, self.x == 0x00);
        self.set_flag(StatusFlags::Negative, (self.x & 0x80) == 0x80);
      }
      OpCode::LDY => {
        self.y = data;
        self.set_flag(StatusFlags::Zero, self.y == 0x00);
        self.set_flag(StatusFlags::Negative, (self.y & 0x80) == 0x80);
      }
      OpCode::LSR => {
        self.set_flag(StatusFlags::Carry, (data & 0x01) == 0x01);
        let temp = data >> 1;
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
        if let AddressingMode::Implied | AddressingMode::Accumulator = mode {
          self.a = temp;
        } else {
          self.write(address, temp);
        }
      }
      OpCode::NOP => (),
      OpCode::ORA => {
        self.a |= data;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::PHA => {
        self.push(self.a);
      }
      OpCode::PHP => {
        self.push((self.status | StatusFlags::Unused | StatusFlags::Break).bits());
        self.set_flag(StatusFlags::Break, false);
        self.set_flag(StatusFlags::Unused, false);
      }
      OpCode::PLA => {
        self.a = self.pop();
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::PLP => {
        self.status = self.pop().try_into().map_err(|e| CpuError::Other(format!("{e}")))?;
        self.set_flag(StatusFlags::Unused, true);
      }
      OpCode::ROL => {
        let carry = self.is_flag(StatusFlags::Carry) as u8;
        let new_carry = (data & 0x80) == 0x80;
        let temp = (data << 1) | carry;
        self.set_flag(StatusFlags::Carry, new_carry);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
        if let AddressingMode::Implied | AddressingMode::Accumulator = mode {
          self.a = temp;
        } else {
          self.write(address, temp);
        }
      }
      OpCode::ROR => {
        let carry = self.is_flag(StatusFlags::Carry) as u8;
        let new_carry = (data & 0x01) == 0x01;
        let temp = (carry << 7) | (data >> 1);
        self.set_flag(StatusFlags::Carry, new_carry);
        self.set_flag(StatusFlags::Zero, temp == 0x00);
        self.set_flag(StatusFlags::Negative, (temp & 0x80) == 0x80);
        if let AddressingMode::Implied | AddressingMode::Accumulator = mode {
          self.a = temp;
        } else {
          self.write(address, temp);
        }
      }
      OpCode::RTI => {
        self.status = self.pop().try_into().map_err(|e| CpuError::Other(format!("{e}")))?;
        self.status.toggle(StatusFlags::Break);
        self.status.toggle(StatusFlags::Unused);

        let lo = self.pop();
        let hi = self.pop();
        self.program_counter = u16::from_le_bytes([lo, hi]);
      }
      OpCode::RTS => {
        let lo = self.pop();
        let hi = self.pop();
        self.program_counter = u16::from_le_bytes([lo, hi]);
      }
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
      OpCode::STA => {
        self.write(address, self.a);
      }
      OpCode::STX => {
        self.write(address, self.x);
      }
      OpCode::STY => {
        self.write(address, self.y);
      }
      OpCode::TAX => {
        self.x = self.a;
        self.set_flag(StatusFlags::Zero, self.x == 0x00);
        self.set_flag(StatusFlags::Negative, (self.x & 0x80) == 0x80);
      }
      OpCode::TAY => {
        self.y = self.a;
        self.set_flag(StatusFlags::Zero, self.y == 0x00);
        self.set_flag(StatusFlags::Negative, (self.y & 0x80) == 0x80);
      }
      OpCode::TSX => {
        self.x = self.stack_ptr;
        self.set_flag(StatusFlags::Zero, self.x == 0x00);
        self.set_flag(StatusFlags::Negative, (self.x & 0x80) == 0x80);
      }
      OpCode::TXA => {
        self.a = self.x;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
      OpCode::TXS => {
        self.stack_ptr = self.x;
      }
      OpCode::TYA => {
        self.a = self.y;
        self.set_flag(StatusFlags::Zero, self.a == 0x00);
        self.set_flag(StatusFlags::Negative, (self.a & 0x80) == 0x80);
      }
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
        return (*value, self.program_counter - 1);
      }
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
    let (addr, overflow) = self.program_counter.overflowing_add(payload);
    if overflow {
      *cycles += 1;
    }
    self.program_counter = addr;
  }
}
