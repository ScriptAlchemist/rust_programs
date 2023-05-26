#[allow(dead_code)]
fn print_all_binaries(n: i32) {for num in 0..=n { println!("{:b}", num); }}
#[allow(dead_code)]
fn print_all_hex_lower(n: i32) {for num in 0..=n { println!("{:x}", num); }}
#[allow(dead_code)]
fn print_all_hex_higher(n: i32) {for num in 0..=n { println!("{:X}", num); }}
#[allow(dead_code)]
fn print_all_octal(n: i32) {for num in 0..=n { println!("{:o}", num); }}
#[allow(dead_code)]
type NanoSecond = u64;
#[allow(dead_code)]
type Inch = u64;
#[allow(dead_code)]
type U64 = u64;

#[derive(Debug)]
#[allow(dead_code)]
struct Number {
    value: i32,
}

#[derive(Debug, PartialEq)]
struct EvenNumber(i32);

impl TryFrom<i32> for EvenNumber {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        if value % 2 == 0 {
            Ok(EvenNumber(value))
        } else {
            Err(())
        }
    }
}

impl From<i32> for Number {
    fn from(value: i32) -> Self {
        Number { value }
    }
}

fn main() {

    // assert_eq!(EvenNumber::try_from(8), Ok(EvenNumber(8)));
    // assert_eq!(EvenNumber::try_from(5), Err(()));

    // let result: Result<EvenNumber, ()> = 8i32.try_into();
    // assert_eq!(result, Ok(EvenNumber(8)));
    // let result: Result<EvenNumber, ()> = 5i32.try_into();
    // assert_eq!(result, Ok(EvenNumber(3)));

    // let num = Number::from(30);
    // println!("my number is {:?}", num);
    // let int = 5;
    // let num: Number = int.into();
    // println!("My number is {:?}", num);
    // let nanoseconds: NanoSecond = 5 as U64;
    // let inches: Inch = 2 as U64;
    // println!("{} nanoseconds + {} inches = {} unit?",
    //     nanoseconds,
    //     inches,
    //     nanoseconds + inches);
    // println!("{number:3>5}", number=1);
    // println!("{number:>5}", number=1);
    // println!("{number:3>width$}", number=2, width=4);
    // println!("{number:2<5}", number=1);
    // println!("{number:3<5}", number=1);
    // print_all_binaries(15);
    // print_all_hex_lower(15);
    // print_all_hex_higher(15);
    // print_all_octal(15);
}
