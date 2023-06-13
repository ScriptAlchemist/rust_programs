fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}
fn check_number(x: Option<i32>) {
    match x {
        Some(x) => {
            println!("number: {:?}", x);
        }
        None => {
            println!("Empty number");
        }
    };
}
fn main() {
    let mut number = plus_one(None);
    check_number(number);

    let five = Some(5);
    number = plus_one(five);
    check_number(number);
}












































