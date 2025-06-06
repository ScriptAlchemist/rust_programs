#[allow(dead_code)]
#[derive(Clone, Copy)]

#[derive(Debug)]
struct Book {
    // `&'static str` is a reference to a string allocated in read only memory
    author: &'static str,
    title: &'static str,
    year: u32,
}

// This function takes a reference to a book
fn borrow_book(book: &Book) {
    println!("I immutably borrowed {} - {} edition", book.title, book.year);
}

// This function takes a reference to a mutable book and changes 'year' to 2014
fn new_edition(book: &mut Book) {
    book.year = 2014;
    println!("I mutably borrowed {} - {} edition", book.title, book.year);
}

fn main() {
    // Create an immutable Book named 'immutabook'
    let immutabook = Book {
        // string literal have type `&'static str`
        author: "Douglas Hofstadter",
        title: "Godel, Escher, Bach",
        year: 1979,
    };

    // Create a mutable copy of `immutabook` and call it `mutabook`
    let mut mutabook = immutabook;

    // Immutably borrow an immutable object
    borrow_book(&immutabook);

    borrow_book(&mutabook);

    new_edition(&mut mutabook);
    mutabook.year = 2000;
    println!("test mutabook: {:?}", mutabook);
    println!("test imutabook: {:?}", immutabook);

    new_edition(&mut mutabook);
}
