fn main (){
    print_numbers(20);
}

fn print_numbers (num: u32) {
    for n in 1..num {
        if is_even(n) {
            println!("{} is ever", n);
        } else {
            println!("{} is odd", n);
        }
    }
}

fn is_even(num: u32) -> bool {
    return num % 2 == 0;
}