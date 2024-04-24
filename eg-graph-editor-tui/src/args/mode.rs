use clap::ValueEnum;
use std::fmt::Display;

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum Mode {
    Editor,
    Proof,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
