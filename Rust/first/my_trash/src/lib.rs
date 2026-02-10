pub use myutils::ferris_say_test;
pub use myutils::inpt::{gen_num, inp_num};

mod chrn;
pub use chrn::test as chrn_test;

mod or;
pub use or::test as or_test;

mod filereader;
pub use filereader::read_username_from_file;

mod reference_cycles;
pub use reference_cycles::{reference_cycles, reference_cycles_2};

pub fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}

mod strct;
pub fn strt() {
    let mut v = vec![strct::Train::new("moscow", strct::Day::Today, [0; 5])];
    v[0].t[0] = -0b0111_1111;

    v.push(strct::Train {
        location: String::from("Bejing"),
        date: strct::Day::Yesterday,
        ..v[0]
    });

    v.push(strct::Train {
        location: String::from("none"),
        ..v[1]
    });
    v[1].set_tomorrow();

    v.into_iter().for_each(|f| f.hello());
}

pub fn slice() {
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);
}

pub fn mtch() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("Match: {max}"),
        _ => (),
    }

    if let Some(max) = config_max {
        println!("if-let: {max}");
    }
}

pub fn find_word(s: &str, index: usize) -> &str {
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

pub fn collect() {
    use std::collections::HashMap;

    for c in "НашЫ игрЫ".chars() {
        print!("{c}");
    }

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    scores.entry(String::from("Yellow")).or_insert(1488);

    let score = scores.get("Blue").copied().unwrap_or(0);
    println!("{}", &score);
    println!("{score:?}");

    scores.insert(String::from("Blue"), 69);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{map:?}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enm() {
        use std::{
            fmt,
            net::{IpAddr, Ipv4Addr, Ipv6Addr},
        };

        #[derive(Debug)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
        }

        impl fmt::Display for SpreadsheetCell {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    SpreadsheetCell::Int(x) => write!(f, "{:?}", x),
                    SpreadsheetCell::Float(x) => write!(f, "{:?}", x),
                    SpreadsheetCell::Text(x) => write!(f, "{:?}", x),
                }
            }
        }

        let localhost_v4 = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
        let localhost_v6 = IpAddr::V6(Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1));

        assert_eq!("127.0.0.1".parse(), Ok(localhost_v4));
        assert_eq!("::1".parse(), Ok(localhost_v6));

        assert_eq!(localhost_v4.is_ipv6(), false);
        assert_eq!(localhost_v4.is_ipv4(), true);

        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");

        let row = vec![
            SpreadsheetCell::Int(3),
            SpreadsheetCell::Text(format!("{s1}-{s2}-{s3}")),
            SpreadsheetCell::Float(10.12),
        ];

        println!("{:?}", row);
    }
}
