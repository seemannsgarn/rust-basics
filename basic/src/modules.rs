

mod dinner {
    fn bear() {
        println!("With a Bear...")
    }

    pub fn drink() {
        println!("You drink a vodka!");
        bear();
    }
    pub mod russian {
        pub fn print_russian(){
            println!("You are the russian!")
        }
    }
}

fn main () {
    dinner::drink();
    dinner::russian::print_russian();
}