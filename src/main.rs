use vcpu::core::{cpu::Cpu, ram::Ram};
use vcpu::core::bus::BusItem;

fn main() {
  let mut ram = Ram::new(0x0000..=0xFFFF);
  ram.write_at_offset(0x8000, &[
    0xA2, 0x0A,
    0x8E, 0x00, 0x00,
    0xA2, 0x03,
    0x8E, 0x01, 0x00,
    0xAC, 0x00, 0x00,
    0xA9, 0x00,
    0x18,
    0x6D, 0x01, 0x00,
    0x88,
    0xD0, 0xFA,
    0x8D, 0x02, 0x00,
    0xEA,
    0xEA,
    0xEA,
  ]);
  ram.write(0xFFFC, 0x00);
  ram.write(0xFFFD, 0x80);

  let mut cpu = Cpu::new(100.0);
  cpu.connect(ram);

  cpu.reset();

  for _ in cpu {
    println!();
  }
}
