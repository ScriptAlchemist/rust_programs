use std::fs::File;
use std::io::prelude::*;
use regex::Regex;

fn main() {
    // This is the path to my ubuntu subsystem
    let path = r"\\wsl$\Ubuntu\home\bender\.lynx_bookmarks.html";

    // Open the bookmark file and read its contents
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(_error) => {
            println!("Error: cannot locate .lynx_bookmarks.html");
            return;
        }
    };

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Use regular expressions to extract the last bookmark link and its title
    let re = Regex::new(r#"<LI><a href="([^"]*)"[^>]*>([^<]*)</a>"#).unwrap();
    let captures = re.captures_iter(&contents).last().unwrap();
    let url = captures[1].to_owned();
    let title = captures[2].to_owned();

    let mut input_buffer = String::new();
    let stdin = std::io::stdin();
    stdin.read_line(&mut input_buffer).unwrap();
    let path = input_buffer.trim();
    print!("{} [{}]({})", path, title, url);

    // Print the last bookmark link and its title to the terminal
    // println!("[{}]({})", title, url);
}
