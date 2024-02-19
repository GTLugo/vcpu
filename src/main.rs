use vcpu::core::{cpu::Cpu, ram::Ram};

fn main() {
  let mut cpu = Cpu::new();
  let ram = Ram::new(0x0000..=0xFFFF);
  cpu.connect(ram);

  cpu.write(0x0069, 0xA1);

  println!("{:#X}", cpu.read(0x0069));
}
