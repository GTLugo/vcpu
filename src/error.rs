use thiserror::Error;

#[derive(Error, Debug)]
pub enum CpuError {
  #[error("op code `{0:02X}` is invalid")]
  InvalidOpCode(u8),
  #[error("program counter overflowed")]
  ProgramCounterOverflow,
  #[error("memory address `0x{0:04X}` is invalid")]
  InvalidAddress(u16),
  #[error("attempted to write to read-only memory address `0x{0:04X}`")]
  WriteToRomAddress(u16),
  #[error("{0}")]
  Other(String),
}
