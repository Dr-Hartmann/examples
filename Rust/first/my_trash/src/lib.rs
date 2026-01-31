#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    fn add(left: u64, right: u64) -> u64 {
        left + right
    }
}

mod or;
use std::io::{self, Read};

pub fn start() {
    'outer: loop {
        let num: u8 = inpt::inp_num();

        match inpt::gen_num(num) {
            Some(result) => {
                let num = result;
                println!("Новое значение задания: {}", num);
            }
            None => {
                println!("Ошибка! (передано '{num}')");
                break 'outer;
            }
        }

        println!("Выбрано: {}", num);
        match num {
            1 => chrn::test(),
            2 => ferris::test(),
            3 => or::test(),
            4 => strct::test(),
            5 => println!("{:?}", find_word("Za warudo!", 3)),
            6 => slice(),
            7 => enm::test(),
            8 => mtch(),
            n => println!("непонятно: {n}"),
        }
    }

    println!("Программа 'trash' выполнена. Нажмите Enter для выхода...");
    let _ = io::stdin().read(&mut [0u8]).unwrap();
}

mod chrn {
    extern crate chrono;
    use chrono::{DateTime, Duration, Utc};

    fn day_earlier(date_time: DateTime<Utc>) -> Option<DateTime<Utc>> {
        date_time.checked_sub_signed(Duration::days(1))
    }

    pub fn test() {
        let now = Utc::now();
        println!("{}", now);

        let almost_three_weeks_from_now = now
            .checked_add_signed(Duration::weeks(2))
            .and_then(|in_2weeks| in_2weeks.checked_add_signed(Duration::weeks(1)))
            .and_then(day_earlier);

        match almost_three_weeks_from_now {
            Some(x) => println!("{}", x),
            None => eprintln!("Almost three weeks from now overflows!"),
        }

        match now.checked_add_signed(Duration::MAX) {
            Some(x) => println!("{}", x),
            None => eprintln!(
                "We can't use chrono to tell the time for the Solar System to complete more than one full orbit around the galactic center."
            ),
        }
    }
}

mod inpt {
    use inquire::Text;
    use rand::Rng;

    pub fn inp_num() -> u8 {
        loop {
            let input = Text::new("Какое задание выполнить?")
                .with_help_message("Введите число")
                .prompt()
                .expect("Неудалось прочитать строку");

            match input.trim().parse() {
                Ok(num) => break num,
                Err(e) => {
                    println!("{e}: Введите нормальное число!");
                    continue;
                }
            }
        }
    }

    pub fn gen_num(num: u8) -> Option<u8> {
        let mut rng = rand::rng();
        let secret_sub = rng.random_range(0..=2);
        num.checked_sub(secret_sub)
    }
}

mod ferris {
    use ferris_says::say;
    use std::io::{BufWriter, stdout};

    pub fn test() {
        let stdout = stdout();
        let message = String::from("Hello fellow Rustaceans!");
        let width = message.chars().count();
        let mut writer = BufWriter::new(stdout.lock());
        say(&message, width, &mut writer).unwrap();
    }
}

fn slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

mod enm {
    use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

    // #[derive(Clone, Copy)]
    // enum IpAddr1 {
    //     V4(u8, u8, u8, u8),
    //     V6(&str),
    // }

    // impl IpAddr1 {
    //     pub fn get<T>(&self) -> Self{
    //         *self
    //     }
    // }

    pub fn test() {
        // let home = IpAddr1::V4(127, 0, 0, 1);
        // let loopback = IpAddr1::V6(String::from("::1"));

        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

        assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
        assert_eq!("::1".parse(), Ok(localhost_v6));

        assert_eq!(localhost_v4.is_ipv6(), false);
        assert_eq!(localhost_v4.is_ipv4(), true);
    }
}

fn mtch() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("Match: {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("if-let: {max}");
    }
}

mod strct {
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
            dbg!(&self);
        }

        pub fn set_tomorrow(&mut self) {
            self.date = Day::Tomorrow;
        }
    }

    pub fn test() {
        let mut t1 = Train::new("moscow", Day::Today, [0; 5]);
        t1.t[0] = -0b0111_1111;

        let t2 = Train {
            location: String::from("Bejing"),
            date: Day::Yesterday,
            ..t1
        };

        let mut t3 = Train {
            location: String::from("none"),
            ..t2
        };
        t3.set_tomorrow();

        for t in [t1, t2, t3] {
            t.hello();
        }
    }
}

fn find_word(s: &str, index: usize) -> &str {
    let bytes = s.as_bytes();
    let mut cur = index + 1;
    let mut start = 0;

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            cur -= 1;
            if cur <= 1 {
                return &s[start..i];
            }
            start = i + 1;
        }
    }

    &s[start..]
}
