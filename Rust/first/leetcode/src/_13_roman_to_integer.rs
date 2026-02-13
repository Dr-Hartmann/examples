use std::collections::HashMap;
use std::sync::LazyLock;

static ROMAN_MAP: LazyLock<HashMap<char, i32>> = LazyLock::new(|| {
    HashMap::from([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ])
});

const ROMAN_SYMBOLS: [(i32, &str); 13] = [
    (1000, "M"), (900, "CM"), (500, "D"), (400, "CD"),
    (100, "C"), (90, "XC"), (50, "L"), (40, "XL"),
    (10, "X"), (9, "IX"), (5, "V"), (4, "IV"), (1, "I"),
];

pub fn roman_to_int(s: &str) -> Result<i32, RomanError> {
    let (mut prev_val, mut res) = (0, 0);
    for c in s.chars().rev() {
        let c_val = ROMAN_MAP
            .get(&c)
            .copied()
            .ok_or(RomanError::InvalidCharacter(c))?;

        if c_val < prev_val {
            res -= c_val;
        } else {
            res += c_val;
        }
        prev_val = c_val;
    }

    Ok(res)
}

pub fn int_to_roman(mut num: i32) -> String {
    let mut res = String::with_capacity(10);

    for (value, symbol) in ROMAN_SYMBOLS {
        while num >= value {
            res.push_str(symbol);
            num -= value;
        }
    }
    res
}

#[derive(Debug, PartialEq)]
pub enum RomanError {
    InvalidCharacter(char),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roman_to_int_test() {
        let cases = [
            ("III", Ok(3)),
            ("IV", Ok(4)),
            ("IX", Ok(9)),
            ("LVIII", Ok(58)),
            ("MCMXCIV", Ok(1994)),
            ("Z", Err(RomanError::InvalidCharacter('Z'))),
        ];

        for (input, expected) in cases {
            let result = roman_to_int(input);
            assert_eq!(result, expected, "Ошибка на входных данных: '{}'", input);
        }
    }

    #[test]
    fn int_to_roman_test() {
        let cases = [
            (3, "III"),
            (4, "IV"),
            (9, "IX"),
            (58, "LVIII"),
            (1994, "MCMXCIV"),
        ];

        for (input, expected) in cases {
            assert_eq!(int_to_roman(input), expected, "Число: {}", input);
        }
    }
}
