use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    pub command: String,
    pub value: u8,
}

/*
impl Cli {
    pub fn new(command: String, value: Option<u8>) -> Self {
        self.command = command;
        self.value = value.unwrap_or(5); // default is 5
    }
}
*/
