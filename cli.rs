pub struct Cli {
    comman: String,
    value: u8,
}

impl Cli {
    pub fn new(comman: String, value: Option<u8>) -> Self {
        self.command = comman;
        self.value = value.unwrap_or(5);
    }
}
