//#![allow(unused)]

mod error;
mod prelude;
mod utils;

use crate::prelude::*;
use std::fs::read_dir;

fn main() -> Result<()> {
    println!("Simple Code Layout");

    for entry in read_dir("./")?.filter_map(|e| e.ok()) {
        let entry: String = W(&entry).try_into()?;
        println!("{entry}");
    }
    Ok(())
}
