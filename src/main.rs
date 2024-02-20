use std::io::stdin;
use vcpu::core::cpu::Cpu;
use vcpu::core::bus::BusItem;
use vcpu::core::ram_file::RamFile;

fn main() {
  let mut ram = RamFile::new(0x0000..=0xFFFF);
  ram.write_all(0x8000, include_bytes!("../assets/a.out"));
  ram.write(0xFFFC, 0x00);
  ram.write(0xFFFD, 0x80);

  let mut cpu = Cpu::new(8.0);
  cpu.connect(ram);

  cpu.reset();

  for _ in cpu {
    stdin().read_line(&mut String::new()).unwrap();
  }
}
