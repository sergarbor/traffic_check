/* -------------------------- APP PARAMS PARSER -------------------------- */
/*
#[derive(Parser)]
pub struct Cli {
    pub command: String,
    pub value: u8,
}

impl Cli {}

*/
use clap::Parser;

#[derive(Parser)]
pub enum Cli {
    Save(SaveArgs),
    Show,
    Output(OutputArgs),
    Protocols(ProtocolsArgs),
    Arpscann(ARPScannArgs),
    // Add more commands as needed
}

#[derive(Parser)]
pub struct SaveArgs {
    #[clap(long, short)]
    pub filename: Option<String>,
}

#[derive(Parser)]
pub struct OutputArgs {
    pub filename: String,
}

#[derive(Parser)]
pub struct ProtocolsArgs {
    pub protocols: Vec<String>,
}

#[derive(Parser)]
pub struct ARPScannArgs {
    #[clap(long, short, action)]
    pub arpscann: bool,
}
