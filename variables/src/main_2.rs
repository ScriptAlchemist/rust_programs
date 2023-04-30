fn main() {
    enum Message {
        WriteToBinary(String),
        WriteToHex(String),
    }

    impl Message {
        fn call(&self) {
            // method body would go here
            match self {
                Message::WriteToBinary(s) => {
                    for byte in s.bytes() {
                        print!("{:08b} ", byte);
                    }
                    println!();
                }
                Message::WriteToHex(s) => {
                    for byte in s.bytes() {
                        print!("{:04x} ", byte);
                    }
                    println!();
                }
            }
        }
    }
    let m = Message::WriteToBinary(String::from("hello"));
    m.call();
    let n = Message::WriteToHex(String::from("hello"));
    n.call();
}





/*
struct QuitMessage; // unit struct
struct MoveMessage {
    x: i32,
    y: i32,
}
struct WriteMessage(String); // tuple struct
struct ChangeColorMessage(i32, i32, i32); // tuple struct
*/
