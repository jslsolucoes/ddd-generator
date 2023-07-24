use clap::{ValueEnum};
pub mod uc;

#[derive(ValueEnum, Clone, Debug)]
pub enum Mode {
    Simulate,
    Execute,
}
