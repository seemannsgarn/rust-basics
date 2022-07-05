extern crate rand; 

use rand::Rng;
fn main () {
    let random_number = rand::thread_rng().gen_range(1..11);

    println!("{}", random_number);

    let random_bool = rand::thread_rng().gen_bool(1.0 / 2.0); // float64
    print!("{}", random_bool)
}