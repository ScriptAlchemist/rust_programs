#![crate_name = "what"]

/// The Vector of numbers represented here
pub struct Array {
    /// The Array of numbers to iterate over
    num: Vec<i32>,
}

impl Array {
    /// # Example
    ///
    /// ```
    /// use what::Array;
    /// let new_array = Array::new([1, 2, 3, 4].to_vec());
    /// ```
    pub fn new(numbers: Vec<i32>) -> Array {
        Array {
            num: numbers,
        }
    }
    /// Example
    /// ```
    /// use what::Array;
    /// let new_array = Array::new([1, 2, 3, 4].to_vec());
    /// assert_eq!(new_array.skip_two(), vec![&3, &4]);
    /// ```
    pub fn skip_two(&self) -> Vec<&i32> {
        self.num.iter().skip(2).collect::<Vec<_>>()
    }
    /// Example
    /// ```
    /// use what::Array;
    /// let new_array = Array::new([1, 2, 3, 4].to_vec());
    /// assert_eq!(new_array.skip_odd(), vec![&2, &4]);
    /// ```
    pub fn  skip_odd(&self) -> Vec<&i32> {
        self.num.iter().filter(|i| *i % 2 == 0).collect::<Vec<_>>()
    }

    pub fn print_it(&self) {
        println!("{:?}", self.num);
    }
}

#[allow(dead_code)]
fn main() {
    let first_array = Array::new([1, 2, 3, 4].to_vec());
    first_array.print_it();
    // Skip the first 2 items
    let skipped_two = first_array.skip_two();
    println!("{:?}", skipped_two); // [3, 4]
    let skipped_odd = first_array.skip_odd();
    println!("{:?}", skipped_odd); // [3, 4]
}
