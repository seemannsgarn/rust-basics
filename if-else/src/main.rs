fn main() {
    let n = 60;

    if n <= 0 {
        println!("Zero or negative number!");
    } else if n > 0 && n < 18 {
        println!("You're the kid, go away!");
    } else if n >= 18 && n < 50 {
        println!("You can drink in this place!");
    } else {
        println!("Why are you too old!");
    }
}
