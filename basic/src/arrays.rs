fn main () {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 3] = [11, 22, 33];
    let arr3 = [100; 2];

    for n in arr1.iter() {
        println!("{}", n)
    }

    for i in 0..arr2.len() {
        println!("{}", arr2[i]);
    }


    for i in 0..arr3.len() {
        println!("{}", arr3[i]);
    }
}