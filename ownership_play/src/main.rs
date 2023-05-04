fn main() {
    let mut s = String::from("hello");
    let mut literal_s = "hello";

    s.push_str(", world!");
    literal_s.push_str(", world!");

    println!("String: {}", s);
    println!("String literal: {}", literal_s);
}
