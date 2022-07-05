fn main () {
    // one way make a vector
    // let my_vector: Vec<i32> = Vec::new();

    let mut my_vector = vec![1,2,3,4];

    my_vector.push(5);
    my_vector.remove(1);

    for num in my_vector.iter(){
        println!("{}", num)
    }
}