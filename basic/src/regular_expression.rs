extern crate regex;
use regex::Regex;

fn main () {
   let re = Regex::new(r"\w{3}").unwrap();
   let re_cap = Regex::new(r"(\w{3})").unwrap();
   let text = "abc";
   
   
   println!("Found match? {}", re.is_match(text));

   match re_cap.captures(text) {
    Some(caps) => println!("Found: {}", caps.get(0).unwrap().as_str()),
    None => println!("not find match...")
   }
}
