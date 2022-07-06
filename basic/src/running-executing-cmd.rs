use::std::process::Command;   

fn main () {
    let mut cmd = Command::new("python3");
    cmd.arg("./../print.py");

    // Execute 
    match cmd.output() {
        Ok(o) => {
            // do stuff here
            unsafe {println!("{}", String::from_utf8_unchecked(o.stdout))}
        }
        Err(e) => {
            println!("Error: {}", e);
        }
    }
}