pub use myutils::ferris_say_test;
pub use myutils::inpt::{gen_num, inp_num};

mod chrn;
pub use chrn::test as chrn_test;

mod or;
pub use or::test as or_test;

mod filereader;
pub use filereader::read_username_from_file;

mod ref_cycles;
pub use ref_cycles::{reference_cycles_1, reference_cycles_2};

mod strct;
pub use strct::{Day, Train};

#[cfg(test)]
mod tests {
    #[test]
    fn slice() {
        let a = [1, 2, 3, 4, 5];
        let slice = &a[1..3];
        assert_eq!(slice, &[2, 3]);
    }

    #[test]
    fn enm() {
        use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

        #[derive(Debug, PartialEq)]
        enum SpreadsheetCell {
            Int(i32),
            Float(f64),
            Text(String),
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

        assert!(
            row.get(2)
                .is_some_and(|x| x == &SpreadsheetCell::Float(10.12))
        );
    }

    #[test]
    fn collect() {
        use std::collections::HashMap;
        let mut scores = HashMap::new();

        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        scores.entry(String::from("Yellow")).or_insert(1488);

        let score = scores.get("Blue").copied().unwrap_or(0);
        assert_eq!(score, 10);

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
}
