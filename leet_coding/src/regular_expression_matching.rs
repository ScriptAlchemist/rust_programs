use std::collections::HashMap;
// 10. Regular Expression Matching: https://leetcode.com/problems/regular-expression-matching/

pub fn compare(
    input: &[u8],
    pattern: &[u8],
    cache: &mut HashMap<(usize, usize), bool>,
) -> bool {
    if let Some(val) = cache.get(&(input.len(), pattern.len())) {
        return *val;
    }

    let val = match (input.get(0), pattern.get(0)) {
        (input_char, Some(pattern_char)) => {
            let char_matched = input_char
                .map(|input_char| *input_char == *pattern_char || *pattern_char == b'.')
                .unwrap_or_default();
            match pattern.get(1) {
                Some(b'*') => {
                    compare(input, &pattern[2..], cache)
                        || (char_matched && compare(&input[1..], &pattern, cache))
                }
                _ => char_matched &&  compare(&input[1..], &pattern[1..], cache),
            }
        }
        (Some(_), None) => false,
        (None, None) => true,
    };
    cache.insert((input.len(), pattern.len()), val);
    val
}
#[allow(dead_code)]
pub fn is_match_v2(s: String, p: String) -> bool {
    let mut cache = HashMap::new();
    compare(s.as_bytes(), p.as_bytes(), &mut cache)
}

#[allow(dead_code)]
pub fn is_match(s: String, p: String) -> bool {
    let mut dp = vec![vec![false; p.len() + 1]; s.len() + 1];
    dp[s.len()][p.len()] = true;
    for i in (0..=s.len()).rev() {
        for j in (0..p.len()).rev() {
            let first_match = i < s.len() &&
                (p.chars().nth(j).unwrap() == s.chars().nth(i).unwrap() ||
                 p.chars().nth(j).unwrap() == '.');
            if j + 1 < p.len() && p.chars().nth(j + 1).unwrap() == '*' {
                dp[i][j] = dp[i][j + 2] || first_match && dp[i + 1][j];
            } else {
                dp[i][j] = first_match && dp[i + 1][j + 1];

            }
        }
    }
    dp[0][0]
}

#[cfg(test)]
mod regular_expression_matching {
    use super::*;

    #[test]
    fn test_0() {
        let s = String::from("aab");
        let p = String::from("c*a*b");
        assert_eq!(is_match(s, p), true);
    }

    #[test]
    fn test_1() {
        let s = String::from("aa");
        let p = String::from("a");
        assert_eq!(is_match(s, p), false);
    }

    #[test]
    fn test_2() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert_eq!(is_match(s, p), true);
    }

    #[test]
    fn test_3() {
        let s = String::from("ab");
        let p = String::from(".*");
        assert_eq!(is_match(s, p), true);
    }

    #[test]
    fn test_4() {
        let s = String::from("aab");
        let p = String::from("c*a*b");
        assert_eq!(is_match_v2(s, p), true);
    }

    #[test]
    fn test_5() {
        let s = String::from("aa");
        let p = String::from("a");
        assert_eq!(is_match_v2(s, p), false);
    }

    #[test]
    fn test_6() {
        let s = String::from("aa");
        let p = String::from("a*");
        assert_eq!(is_match_v2(s, p), true);
    }

    #[test]
    fn test_7() {
        let s = String::from("ab");
        let p = String::from(".*");
        assert_eq!(is_match_v2(s, p), true);
    }
}
