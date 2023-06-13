mod tryfrom_and_tryinto;
mod to_and_from_strings;
mod show_colors;
mod expressions;

fn main() {
    println!("\x1b[37m\x1b[41mMain Starting\x1b[0m\n");
    // show_colors::runner();
    // to_and_from_strings::runner();
    // tryfrom_and_tryinto::runner();
    expressions::runner();
}
