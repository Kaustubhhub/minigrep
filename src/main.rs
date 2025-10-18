use std::{env, fs};
fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);

    let query = &args[1];
    let file_path = &args[2];

    let content = fs::read_to_string(file_path).expect("unable to read from this path");

    println!("{:?}", {content});
}
