use std::fs::File;
use std::io::prelude::*;

fn main () {
    let mut file = File::create("./../output.txt")
        .expect("cant create a file");

    file.write_all(b"Hello youtube!!!")
        .expect("cant write in a file");
}