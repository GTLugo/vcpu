use std::io::stdin;
use vcpu::core::cpu::Cpu;
use vcpu::core::ram_file::RamFile;

fn main() {
  let mut ram = RamFile::new(0x0000..=0xFFFF);
  ram.write_all(0x0000, include_bytes!("../assets/a.out"));

  let mut cpu = Cpu::new(100.0);
  cpu.connect(ram);

  cpu.reset();

  for _ in cpu {
    stdin().read_line(&mut String::new()).unwrap();
  }
}
