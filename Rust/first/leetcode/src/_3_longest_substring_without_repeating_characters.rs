pub fn length_of_longest_substring(str: &str) -> i32 {
    solve(String::from(str))
}

// метод скользящего окна (sliding window)
use std::collections::HashSet;
fn solve(s: String) -> i32 {
    let mut set: HashSet<char> = HashSet::new();
    let chars: Vec<char> = s.chars().collect();
    let (mut tail, mut res) = (0, 0);

    for head in 0..chars.len() {
        while set.contains(&chars[head]) {
            set.remove(&chars[tail]);
            tail += 1;
        }
        set.insert(chars[head]);
        res = res.max(head - tail + 1);
    }
    res as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_substring_without_repeating_characters() {
        assert_eq!(length_of_longest_substring("abcabcbb"), 3); // "abc"
        assert_eq!(length_of_longest_substring("bbbbb"), 1); // "b"
        assert_eq!(length_of_longest_substring("pwwkew"), 3); // "wke"

        // Краевые случаи
        assert_eq!(length_of_longest_substring(""), 0); // Пустая строка
        assert_eq!(length_of_longest_substring(" "), 1); // Пробел (ваш trim() его удалит!)
        assert_eq!(length_of_longest_substring("au"), 2); // Нет повторов
        assert_eq!(length_of_longest_substring("dvdf"), 3); // Сложный случай для HashSet.clear()

        // Символы и цифры
        assert_eq!(length_of_longest_substring("aab"), 2);
        assert_eq!(length_of_longest_substring("123121"), 3);
    }
}
