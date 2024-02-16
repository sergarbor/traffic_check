use clap::Parser;

/* -------------------------- APP PARAMS PARSER -------------------------- */
#[derive(Parser)]
pub struct Cli {
    pub command: String,
    pub value: u8,
}

impl Cli {}
