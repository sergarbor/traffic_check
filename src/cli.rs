use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub command: String,
    pub value: u8,
}
