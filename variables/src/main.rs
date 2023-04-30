#![crate_name = "what"]

fn main() {
    let c = "Z";
    let z: char = 'Z';
    let equal_or_not = c == z.to_string();
    println!("Is {} and {} equal? {}", z, c, equal_or_not);
}

















/*
pub struct Number {
    number: i32,
}

impl Number {

    pub fn new(num: i32) -> Number {
        Number {
            number: num,
        }
    }
    pub fn add_five(&self) -> i32 {
        self.number + 5
    }
}


fn main() {
    let the_num = Number::new(5);
    println!("Add Five: {}", the_num.add_five());
}



struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/
