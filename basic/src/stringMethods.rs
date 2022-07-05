fn main () {
    // Replace
    {
        let my_string = String::from("Vader on the dark side...");
        println!("After Replace: {}", my_string.replace("dark", "light"));
    }

    // Lines
    {
        let my_string = String::from("Luke I am...\nyour father. \nNoooooo!");
        for line in my_string.lines() {
            println!("[ {} ]", line);
        }
    }

    // Split
    {
        let my_string = String::from("This+is+the+way+.");
        let tokens: Vec<&str> = my_string.split("+").collect();

        println!("{}", my_string);
        println!("Index 2 equals: {}", tokens[3]);
        
    }
    // Trim
    {
        let my_string = String::from("       This guy says hello. I wanna kill myself. \n\r");
        println!("Before: {}", my_string);
        println!("After: {}", my_string.trim());
    }
    // Chars
    {
        let my_string = String::from("Darth Vader");
        println!("{}", my_string);

        match my_string.chars().nth(6) {
            Some(c) => println!("Char is: {}", c),
            None => println!("No character at index 4")
        }
    }
}