struct Color {
    red: u8, // 0-255
    green: u8,
    blue: u8
}

fn main (){

    let mut bg = Color {red: 255, green: 56, blue: 124};

    bg.green = 188;

    println!("{}, {}, {}", bg.red, bg.green, bg.blue);
}

