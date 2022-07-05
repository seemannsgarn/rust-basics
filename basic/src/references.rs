fn main (){
    let mut x = 100;
    let rx = &x;    // reference 
    println!("rx == {}", rx);

    {
    // change var
    let y = &mut x;
    *y += 23;
    }

    println!("x == {}", x);
    
}

