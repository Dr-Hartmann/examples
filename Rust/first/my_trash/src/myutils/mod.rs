mod frris;
pub use frris::test as ferris_say_test;
mod strct;
pub mod or;
pub mod filereader;

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

pub fn strct() {
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

    for t in &v {
        t.hello();
    }

    // v.into_iter(|x| -> x.hello());
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
