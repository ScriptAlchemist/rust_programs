
fn main() {
    let c: &str = "😻"; // Creates an implied String
    println!("cat bytes: {:?}", c.as_bytes());

    // turn c into a char
    let new_c: char = c.chars().next().unwrap();




    let z: char = '😻'; // Creates an implied char

    println!("Value of the char: {}", z as u32);
    let equal_or_not = c == z.to_string() && new_c == z; // Convert the char to string to compare
    println!("Is {} and {} equal? {}", z, c, equal_or_not);

    // turn the char into a byte slice
    let mut bytes = [0_u8; 4];
    let mut utf8_bytes = [0_u8; 4];
    bytes.copy_from_slice(&z.encode_utf8(&mut utf8_bytes).as_bytes());
    println!("Byte representation of the char: {:?}", bytes);
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
