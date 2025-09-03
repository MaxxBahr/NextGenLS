use std::env;

pub struct Arguments {
    short: String,
    long: String,
    value: Option<String>
}

impl Arguments {
    pub fn short(&mut self, value: String){
        self.short = value;
    }

    pub fn long(&mut self, value: String){
        self.long = value;
    }
}