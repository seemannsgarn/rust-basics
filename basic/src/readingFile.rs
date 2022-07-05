use std::fs::File;
use std::io::prelude::*;

fn main () {
  let mut file = File::open("./../test.txt").expect("Can't open the file!");

  let mut content = String::new();
  file.read_to_string(&mut content)
    .expect("Can't reading the file!");

    println!("File content:\n\n{}",content);
}