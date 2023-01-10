use std::env;
use std::fs;
use std::string::String;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short)]
    snippet: String,

    #[arg(short)]
    name: Option<String>,
    
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    // Get the location of the snippets folder from the environment variable
    let snippets_location = env::var("SNIPPETS_LOCATION").unwrap_or_else(|_| {
        println!("SNIPPETS_LOCATION environment variable not set");
        std::process::exit(1);
    });

    // Read the snippet file
    let mut snippet = fs::read_to_string(format!("{}/{}", snippets_location, args.snippet)).unwrap();

    if let Some(name) = args.name {
        // Replace all occurrences of ":%:" with the name
        snippet = snippet.replace(":%:", &name);
    }

    // Print the snippet to stdout the specified number of times
    for _ in 0..args.count {
        println!("{}", snippet);
    }
}

