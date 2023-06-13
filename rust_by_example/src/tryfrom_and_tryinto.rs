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
#[allow(dead_code)]
pub fn runner() {
    println!("{}", format!("\n\x1b[33m\x1b[47mstart tryfrom_and_tryinto\x1b[0m\n"));
    println!("{:?}", EvenNumber::try_from(8));
    println!("{:?}", EvenNumber::try_from(5));
    let result: Result<EvenNumber, ()> =8i32.try_into();
    println!("{:?}", result);
    let result: Result<EvenNumber, ()> =5i32.try_into();
    println!("{:?}", result);
}
