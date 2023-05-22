#[allow(dead_code)]
pub fn longest_palindrome_v1(s: String) -> String {
    if s.len() < 2 {
        return s;
    }

    let s = s.chars().collect::<Vec<char>>();
    let (mut start, mut end) = (0, 0);

    for i in 0..s.len() {
        let (mut left, mut right) = (i, i);
        while right < s.len() - 1 && s[right] == s[right + 1] {
            right += 1;
        }
        while left > 0 && right < s.len() - 1 && s[left - 1] == s[right + 1] {
            left -= 1;
            right += 1;
        }
        if right - left > end - start {
            start = left;
            end = right;
        }
    }
    s[start..=end].iter().collect::<String>()
}

#[cfg(test)]
mod longest_palindrome_v1 {
    use super::*;
    #[test]
    fn test_1() {
        let input = String::from("kscriptpircsd");
        let output = String::from("scriptpircs");
        assert_eq!(longest_palindrome_v1(input), output);
    }

    #[test]
    fn test_2() {
        let input = String::from("tcrablangnalbarcd");
        let output = String::from("crablangnalbarc");
        assert_eq!(longest_palindrome_v1(input), output);
    }
    #[test]
    fn test_3() {
        let input = String::from("bjds");
        let output = String::from("b");
        assert_eq!(longest_palindrome_v1(input), output);
    }
}

#[allow(dead_code)]
pub fn longest_palindrome_v2(s: String) -> String {
    let len = s.len();
    if len < 2 {
        return s;
    }

    let s = s.chars().collect::<Vec<char>>();
    let mut dp = vec![vec![false; len]; len];
    let (mut left, mut right) = (0, 0);

    for j in 1..len {
        for i in 0..j {
            if s[i] == s[j] && (j - i <= 2 || dp[i + 1][j - 1]) {
                dp[i][j] = true;
            }
            if dp[i][j] && (right - left) < (j - i) {
                left = i;
                right = j;
            }
        }
    }
    s[left..=right].iter().collect()
}

#[cfg(test)]
mod longest_palindrome_v2 {
    use super::*;
    #[test]
    fn test_1() {
        let input = String::from("kscriptpircsd");
        let output = String::from("scriptpircs");
        assert_eq!(longest_palindrome_v1(input), output);
    }

    #[test]
    fn test_2() {
        let input = String::from("tcrablangnalbarcd");
        let output = String::from("crablangnalbarc");
        assert_eq!(longest_palindrome_v1(input), output);
    }
    #[test]
    fn test_3() {
        let input = String::from("bjds");
        let output = String::from("b");
        assert_eq!(longest_palindrome_v1(input), output);
    }
}

fn longest_palindrome_from_idx(input: &[u8], mut left: usize, mut right: usize) -> &[u8] {
    while left.checked_sub(1).is_some()
        && right + 1 < input.len()
        && input[left - 1] == input[right + 1] {
        left -= 1;
        right += 1;
    }
    &input[left..=right]

}
#[allow(dead_code)]
pub fn longest_palindrome_v3(s: String) -> String {
    let s = s.as_bytes();
    let mut longest_palindrome = &s[0..1];

    for i in 0..(s.len() - 1) {
        let longest_odd = longest_palindrome_from_idx(s, i, i);
        if longest_odd.len() > longest_palindrome.len() {
            longest_palindrome = longest_odd;
        }

        if s[i] == s[i + 1] {
            let longest_even = longest_palindrome_from_idx(s, i, i + 1);
            if longest_even.len() > longest_palindrome.len() {
                longest_palindrome = longest_even;
            }
        }

    }
    String::from_utf8(longest_palindrome.to_vec()).unwrap()
}

#[cfg(test)]
mod longest_palindrome_v3 {
    use super::*;
    #[test]
    fn test_1() {
        let input = String::from("kscriptpircsd");
        let output = String::from("scriptpircs");
        assert_eq!(longest_palindrome_v3(input), output);
    }

    #[test]
    fn test_2() {
        let input = String::from("tcrablangnalbarcd");
        let output = String::from("crablangnalbarc");
        assert_eq!(longest_palindrome_v3(input), output);
    }
    #[test]
    fn test_3() {
        let input = String::from("bjds");
        let output = String::from("b");
        assert_eq!(longest_palindrome_v3(input), output);
    }
}
