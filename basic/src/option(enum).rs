fn main() {
    println!("Occupation is {}", match get_occupation("Mando") {
        Some(o) => o,
        None => "No occupation found"
    });
}
fn get_occupation(name: &str) -> Option<&str> {
    match name {
        "Mando" => Some("The mandalorian"),
        "Luke" => Some("The jedi"),
        _ => None
    }
}
