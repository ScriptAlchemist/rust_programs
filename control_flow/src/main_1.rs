fn main() {
    println!("Let's start the loops!");
    let mut count = 0;
    'outer_loop: loop {
        println!("count = {count}");
        let mut remaining = 10;

        'inner_loop: loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break 'inner_loop;
            }
            if count == 2 {
                println!("Break the loops");
                break 'outer_loop;
            }
            remaining -= 1;
        }
        count += 1;
    }
    println!("End count = {count}");
}
