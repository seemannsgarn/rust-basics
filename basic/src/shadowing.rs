fn main (){
    let mut s = "Vader";
    {
        let s = "Luke";
    }

    let s = 123;
    println!("s = {}", s);

    let s = true;
    println!("s = {}", s);
}

