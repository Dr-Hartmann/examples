use ferris_says::say;
use std::io::{BufWriter, stdout};

#[derive(Debug)]
pub struct Train {
    location: String,
    date: usize,
    t: i16,
}

impl Train {
    pub fn new(location: &str, date: usize, t: i8) -> Self {
        Self {
            location: location.to_string(),
            date,
            t: (t as i16) << 2,
        }
    }

    pub fn hello(&self) {
        println!("Hello, {}:{}:{}!", self.location, self.date, self.t);
        println!("{:?}!", self);

        let stdout = stdout();
        let message = String::from("Hello fellow Rustaceans!");
        let width = message.chars().count();

        let mut writer = BufWriter::new(stdout.lock());
        say(&message, width, &mut writer).unwrap();
    }
}
