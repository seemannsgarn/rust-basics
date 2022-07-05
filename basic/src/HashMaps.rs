use std::collections::HashMap;

fn main () {
    let mut marks = HashMap::new();

    // add values
    marks.insert("Darth Vader", "Dark Side");
    marks.insert("Luke Skywalker", "Light Side");
    marks.insert("Yoda", "Light Side");
    marks.insert("Darth Mol", "Dark Side");

    // find length of HashMap
    println!("Length of HaspMap: {}", marks.len());

    // get a single value
    match marks.get("Darth Vader") {
        Some(mark) => println!("{}", mark), 
        None => println!("There is nothing")
    }

    // remove mark
    marks.remove("Yoda");

    // lopp
    for (subject, mark) in &marks {
        println!("Who: {} Side: {}", subject, mark)
    }

    // check for value
    println!("Yoda is here? {}", marks.contains_key("Yoda"));

}