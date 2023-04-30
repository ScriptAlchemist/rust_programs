#![crate_name = "benchmark_test"]

pub fn get_first_word_passing_ownership_of_string(param: String) -> String {
    let mut iter = param.split_whitespace();
    iter.next().unwrap_or("").to_string()
}

pub fn get_first_word_borrowing_string(param: &String) -> &str {
    let mut iter = param.split_whitespace();
    iter.next().unwrap_or("")
}

pub fn get_first_word_borrowing_slice(param: &str) -> &str {
    let chars = param.chars();
    for (i, item) in chars.enumerate() {
        if item == ' ' {
            return &param[..i];
        }
    }
    &param[..]
}

pub fn get_first_word_borrowing_slice_v2(s: &str) -> &str {
    s.split_whitespace().next().unwrap_or("")
}

pub fn get_first_word_byte_loop_borrowing_slice(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &byte) in bytes.iter().enumerate() {
        if byte == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

#[allow(dead_code)]
fn main() {
    println!("Hello");
}
