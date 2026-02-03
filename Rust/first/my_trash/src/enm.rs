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

pub fn enm() {
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
