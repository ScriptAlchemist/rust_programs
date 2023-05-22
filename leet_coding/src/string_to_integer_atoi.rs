// 8. String to Integer Atoi: https://leetcode.com/problems/string-to-integer-atoi/
#[allow(dead_code)]
pub fn my_atoi(s: String) -> i32 {
    let v: Vec<u8> = s.bytes().collect();
    let mut idx = 0;
    let len = v.len();
    let mut sign = 1;

    while idx < len && v[idx] == b' ' {
        idx += 1;
    }

    if idx < len && (v[idx] == b'-' || v[idx] == b'+') {
        if v[idx] == b'-' {
          sign = -1;
        }
        idx += 1;
    }

    let mut num: i32 = 0;
    while idx < len && v[idx] >= b'0' && v[idx] <= b'9' {
        let digit = (v[idx] - b'0') as i32;
        if num > std::i32::MAX / 10 || (num == std::i32::MAX / 10 && digit > 7) {
            return if sign == 1 { std::i32::MAX } else { std::i32::MIN };
        }
        num = num * 10 + digit;
        idx += 1;
    }
    num * sign
}
#[cfg(test)]
mod my_atoi_tests {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(my_atoi(String::from("42")), 42);
    }
    #[test]
    fn test_2() {
        assert_eq!(my_atoi(String::from("   -42")), -42);
    }
    #[test]
    fn test_3() {
        assert_eq!(my_atoi(String::from("4193 with words")), 4193);
    }
}

#[allow(dead_code)]
pub fn my_atoi_v2(s: String) -> i32 {
    let s = s.trim_start();
    let (s, sign) = match s.strip_prefix('-') {
        Some(s) => (s, -1),
        None => (s.strip_prefix('+').unwrap_or(s), 1)
    };

    s.chars()
        .map(|c| {c.to_digit(10)})
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |acc, num| {
            acc.saturating_mul(10).saturating_add(sign * num as i32)
        })
}
#[cfg(test)]
mod my_atoi_tests_v2 {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(my_atoi_v2(String::from("42")), 42);
    }
    #[test]
    fn test_2() {
        assert_eq!(my_atoi_v2(String::from("   -42")), -42);
    }
    #[test]
    fn test_3() {
        assert_eq!(my_atoi_v2(String::from("4193 with words")), 4193);
    }
}

#[allow(dead_code)]
pub fn my_atoi_v3(s: String) -> i32 {
    let mut sign = 1;
    let mut num: i32 = 0;
    let bytes: Vec<u8> = s.trim_start().bytes().collect();

    let mut idx = 0;
    if bytes.is_empty() {
        return 0;
    }

    match bytes[0] {
        b'-' => {
            sign = -1;
            idx += 1;
        }
        b'+' => {
            idx += 1;
        }
        _ => {}
    }

    while idx < bytes.len() && bytes[idx].is_ascii_digit() {
        let digit = (bytes[idx] - b'0') as i32;
        if num > std::i32::MAX / 10 || (num == std::i32::MAX / 10 && digit > 7) {
            return match sign {
                1 => std::i32::MAX,
                _ => std::i32::MIN,
            };
        }
        num = num * 10 + digit;
        idx += 1;
    }
    num * sign
}
#[cfg(test)]
mod my_atoi_tests_v3 {
    use super::*;
    #[test]
    fn test_1() {
        assert_eq!(my_atoi_v3(String::from("42")), 42);
    }
    #[test]
    fn test_2() {
        assert_eq!(my_atoi_v3(String::from("   -42")), -42);
    }
    #[test]
    fn test_3() {
        assert_eq!(my_atoi_v3(String::from("4193 with words")), 4193);
    }
}
