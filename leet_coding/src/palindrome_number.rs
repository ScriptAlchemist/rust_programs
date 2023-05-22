// 9. Palindrome Number: https://leetcode.com/problems/palindrome-number

pub fn is_palindrome(x: i32) -> bool {
    if x < 0 { return false; }
    let num_str = x.to_string();
    let reversed_str: String = num_str.chars().rev().collect();
    num_str == reversed_str
}

pub fn is_palindrome_v2(x: i32) -> bool {
    if x < 0 { return false; }
    let num_str = x.to_string();
    let bytes = num_str.as_bytes();
    let length = bytes.len();
    for i in 0..length / 2 {
        if bytes[i] != bytes[length - 1 - i] {
            return false;
        }
    }
    true
}

pub fn is_palindrome_v3(x: i32) -> bool {
    if x < 0 || (x != 0 && x % 10 == 0) {
        return false;
    }
    let mut result = 0;
    let mut num = x;
    while num > 0 {
        let digit = num % 10;
        result = result * 10 + digit;
        num /= 10;
    }
    result == x
}

#[cfg(test)]
mod is_palindrome_number {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(is_palindrome(121), true);
    }
    #[test]
    fn test_2() {
        assert_eq!(is_palindrome(-121), false);
    }
    #[test]
    fn test_3() {
        assert_eq!(is_palindrome(10), false);
    }
    #[test]
    fn test_4() {
        assert_eq!(is_palindrome(11), true);
    }
    #[test]
    fn v2_test_1() {
        assert_eq!(is_palindrome_v2(121), true);
    }
    #[test]
    fn v2_test_2() {
        assert_eq!(is_palindrome_v2(-121), false);
    }
    #[test]
    fn v2_test_3() {
        assert_eq!(is_palindrome_v2(10), false);
    }
    #[test]
    fn v3_test_1() {
        assert_eq!(is_palindrome_v3(121), true);
    }
    #[test]
    fn v3_test_2() {
        assert_eq!(is_palindrome_v3(-121), false);
    }
    #[test]
    fn v3_test_3() {
        assert_eq!(is_palindrome_v3(10), false);
    }
}
