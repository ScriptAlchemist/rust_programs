// 7. Reverse Integer: https://leetcode.com/problems/reverse-integer/

fn flip_abs_integer(int: i32) -> i32 {
    let int_str = int.to_string();
    let reversed_str: String = int_str.chars().rev().collect();

    match reversed_str.parse::<i32>() {
        Ok(reversed_int) => reversed_int,
        Err(_) => 0,
    }
}

#[allow(dead_code)]
pub fn reverse(x: i32) -> i32 {
    let is_negative = x.is_negative();

    match is_negative {
        true => -flip_abs_integer(x.abs()),
        _ => flip_abs_integer(x),
    }
}

#[cfg(test)]
mod reverse_integer_v1 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse(120), 21);
    }
}

#[allow(dead_code)]
pub fn reverse_v2(x: i32) -> i32 {
    let mut result: i32 = 0;
    let mut int = x;
    let sign = if int < 0 { -1 } else { 1 };

    // Remove sign
    int *= sign;

    while int != 0 {
        let digit = int % 10;
        // If overflow occurs, return 0
        result = match result.checked_mul(10) {
            Some(val) => val,
            None => return 0,
        };
        result = match result.checked_add(digit) {
            Some(val) => val,
            None => return 0,
        };
        int /= 10;
    }
    result * sign // add sign back
}

#[cfg(test)]
mod reverse_integer_v2 {
    use super::*;

    #[test]
    fn test_1() {
        assert_eq!(reverse_v2(123), 321);
    }

    #[test]
    fn test_2() {
        assert_eq!(reverse_v2(-123), -321);
    }

    #[test]
    fn test_3() {
        assert_eq!(reverse_v2(120), 21);
    }
}





