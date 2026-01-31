use std::fmt;

#[derive(Debug, Clone, Copy)]
enum Day {
    Yesterday,
    Today,
    Tomorrow,
}

impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let name = match self {
            Day::Yesterday => "Вчера",
            Day::Today => "Сегодня",
            Day::Tomorrow => "Завтра",
        };
        write!(f, "{}", name)
    }
}

#[derive(Debug, Clone)]
struct Train {
    location: String,
    date: Day,
    t: [i16; 5],
}

impl Train {
    pub fn new(location: &str, date: Day, t: [i8; 5]) -> Self {
        Self {
            location: location.to_string(),
            date,
            t: t.map(|x| (x as i16) << 2),
        }
    }

    pub fn hello(&self) {
        let tuple = (&self.location, self.date.clone(), self.t);
        println!("Hello, {:?}!", tuple);
        println!("Время рейса: {:?}", tuple.1);
    }
}

pub fn test() {
    let mut train = Train::new("moscow", Day::Today, [0; 5]);
    train.t[0] = -0b0111_1111;

    train.hello();
}
