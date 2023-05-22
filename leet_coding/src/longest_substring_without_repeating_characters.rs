use std::collections::HashMap;
use std::cmp;

pub fn length_of_longest_substring(s: String) -> i32 {
    // Setup letter HashMap
    let mut map: HashMap<char, usize> = HashMap::new();
    let mut max_length = 0;
    let mut start = 0;

    for (end, c) in s.chars().enumerate() {
        if map.contains_key(&c) {
            start = cmp::max(*map.get(&c).unwrap() + 1, start);
        }
        max_length = cmp::max(max_length, end - start + 1);
        map.insert(c, end);
    }
    max_length as i32
}

#[cfg(test)]
mod v1_length_of_longest_substring {
    use super::*;

    #[test]
    fn length_of_longest_substring_v1_1() {
        let s = String::from("abcabcbb");
        let output = 3;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn length_of_longest_substring_v1_2() {
        let s = String::from("bbbbb");
        let output = 1;
        assert_eq!(length_of_longest_substring(s), output);
    }

    #[test]
    fn length_of_longest_substring_v1_3() {
        let s = String::from("pwwkew");
        let output = 3;
        assert_eq!(length_of_longest_substring(s), output);
    }
}


#[allow(dead_code)]
pub fn length_of_longest_substring_v2(s: String) -> i32 {
    let (mut start, mut max_length, s) = (0, 0, s.into_bytes().into_boxed_slice());

    for (end, c) in s.iter().enumerate() {
        let window = &s[start..end];
        if let Some(idx) = window.iter().position(|cc| c == cc) {
            start += idx + 1;
            max_length = max_length.max(window.len());
        }
    }

    max_length = max_length.max(s.len() - start);

    max_length as i32
}

#[cfg(test)]
mod v2_length_of_longest_substring {
    use super::*;

    #[test]
    fn length_of_longest_substring_v2_1() {
        let s = String::from("abcabcbb");
        let output = 3;
        assert_eq!(length_of_longest_substring_v2(s), output);
    }

    #[test]
    fn length_of_longest_substring_v2_2() {
        let s = String::from("bbbbb");
        let output = 1;
        assert_eq!(length_of_longest_substring_v2(s), output);
    }

    #[test]
    fn length_of_longest_substring_v2_3() {
        let s = String::from("pwwkew");
        let output = 3;
        assert_eq!(length_of_longest_substring_v2(s), output);
    }
}
