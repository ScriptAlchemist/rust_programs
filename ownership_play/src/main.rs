fn main() {
    let s1 = String::from("hello");
    let literal_s_1 = "hello";
    let s2 = s1.clone();
    let literal_s_2 = literal_s_1;

    println!("String 1: {}", s1);
    println!("String 2: {}", s2);
    println!("String literal 1: {}", literal_s_1);
    println!("String literal 2: {}", literal_s_2);
}
