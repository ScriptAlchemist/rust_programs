// Let's think about tuples and what they can do. 
//
// Can they be compared and equal.
//
// They cannot get larger in length

#[allow(dead_code)]
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    NewJersey,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickle,
    Dime,
    Quarter(UsState),
}

#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky Penny");
            1
        }
        Coin::Nickle => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {

    // Get the value back of a coin from enum matching
    let alaska_quarter = value_in_cents(Coin::Quarter(UsState::Alaska));
    println!("The alaska quarter is worth: {}", alaska_quarter);
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    println!("What is six: {:?}, what is none: {:?}", six, none);

}
