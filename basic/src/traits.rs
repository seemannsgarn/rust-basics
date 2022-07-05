
// traits like an interface
struct Person {
    name: String,
    age: u8
}
impl ToString for Person {
    fn to_string(&self) -> String {
        return format!("Wanted {}, {} year old", self.name, self.age)
    }
}

fn main () {
    let driver1 = Person {name: String::from("Brian O'Conner"), age: 35};

    println!("{}", driver1.to_string());
}