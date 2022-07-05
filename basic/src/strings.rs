fn main () {
    let mut my_str = String::from("Hello Youtube! My name is Snicky!");

    println!("Lenght: {}", my_str.len());

    my_str.push_str(" I'am ADC!");
    println!("{}",my_str);

    for token in my_str.split_whitespace() {
        println!("{}",token)
    }

    println!("This is string is empty?: {}",my_str.is_empty());
    println!("Contains Snicky: {}", my_str.contains("Snicky"));
}