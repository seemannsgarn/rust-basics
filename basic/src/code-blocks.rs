fn main (){
    let x = 10;

    // isolated
    {
        let y = 5;
        // this is work
        println!("x: {} y: {}", x, y);
    }
    // this is not work
    // println!("x: {} y: {}", x, y)
}

