use clap::{Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum CliRule {
    Default,
}

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    pub path: String,
    pub rule: Option<CliRule>,
}
