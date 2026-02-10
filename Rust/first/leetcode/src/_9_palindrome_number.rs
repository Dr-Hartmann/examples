pub fn is_palindrome_math(x: i32) -> bool {
    // Отрицательные числа и числа, заканчивающиеся на 0 (кроме самого 0),
    // не могут быть палиндромами.
    if x < 0 || (x % 10 == 0 && x != 0) {
        return false;
    }

    let mut reverted_number = 0;
    let mut temp_x = x;

    // Разворачиваем только вторую половину числа.
    // Когда temp_x станет меньше или равен reverted_number, мы дошли до середины.
    while temp_x > reverted_number {
        reverted_number = reverted_number * 10 + temp_x % 10;
        temp_x /= 10;
    }

    temp_x == reverted_number || temp_x == reverted_number / 10
}

pub fn is_palindrome_string(x: i32) -> bool {
    if x < 0 {
        return false;
    }

    let s = x.to_string();
    let bytes = s.as_bytes();
    let mut left = 0;
    let mut right = bytes.len() - 1;

    while left < right {
        if bytes[left] != bytes[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_palindrome() {
        // математическое решение
        assert!(is_palindrome_math(121));
        assert!(is_palindrome_math(1));
        assert!(is_palindrome_math(1221));
        assert!(is_palindrome_math(0));

        assert!(!is_palindrome_math(-121));
        assert!(!is_palindrome_math(-1));

        assert!(!is_palindrome_math(10));
        assert!(!is_palindrome_math(123));
        assert!(!is_palindrome_math(12322));

        assert!(is_palindrome_math(1234565432));
        assert!(!is_palindrome_math(2147483647));

        // решение через строки
        assert!(is_palindrome_string(121));
        assert!(is_palindrome_string(1));
        assert!(is_palindrome_string(1221));
        assert!(is_palindrome_string(0));

        assert!(!is_palindrome_string(-121));
        assert!(!is_palindrome_string(-1));

        assert!(!is_palindrome_string(10));
        assert!(!is_palindrome_string(123));
        assert!(!is_palindrome_string(12322));

        assert!(is_palindrome_string(1234565432));
        assert!(!is_palindrome_string(2147483647));
    }
}
