fn main (){
    let tup1 = (10, 20, 30, 40);
    let tup2 = (10,"Rust",3.45,true);
    let tup3 = (10,20,30,("Hello","Youtube"));
    let tup4 = ("Rust","Go","C++");
    let (a, b, c) = tup4;

    println!("{}", tup1.2);
    println!("{}", tup2.3);
    println!("{}", (tup3.3).1);
    println!("a is {}", a);
    println!("b is {}", b);
    println!("c is {}", c);
}