struct Color {
    red: u8,
    green: u8,
    blue: u8
}

fn main() {
    let green = Color {red:0, green:254, blue:0};

    print_color(&green);
    print_color(green);
}

fn print_color(c: &Color){
    println!("Color - R:{} G:{} B:{}",c.red, c.green, c.blue);
}
