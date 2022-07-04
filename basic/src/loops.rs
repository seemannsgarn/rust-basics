fn main() {
    // simple_loop(0);
    // println!("====================");
    // while_loop(10);
    // println!("====================");
    for_loop();
}

// fn simple_loop(number: i32) {
//     let mut n = number;
//     loop {
//         n += 1;
//         if n == 7 {
//             continue;
//         }
//         if n == 8 {
//             println!("О_О");
//         }
//         if n > 10 {
//             break;
//         }
//         println!("The value of n is {}", n);
//     }
// }

// fn while_loop(num: i32) {
//     let mut n = num;

//     while n <= 50 {
//         //if n is a multiple of 5
//         if n % 5 == 0 {
//             println!("n is {}", n);
//         }
//         n += 1;
//     }
// }

fn for_loop() {

    let jedies = vec!["Yoda", "Kenobi", "Windu", "Skywalker"];

    for (index, j) in jedies.iter().enumerate() {
        println!("Index: {} and Value: {}", index, j);
    }

    // let numbers = 100..115;
    // for i in numbers {
    //     println!("{}",i);
    // }

    // for i in 1..n {
    //     println!("The number is {}", i)
    // }
}