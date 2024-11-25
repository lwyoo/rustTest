// fn main() {
//     println!("Hello, world!");

//     let num = 3;

//     if num < 10 {
//         println!("cibdutuib was true");
//     } else {
//         println!("cibdutuib was false");
//     }
// }


// fn main() {
//     let num = 3;

//     if (num % 2) == 0 {
//         println!("number is divisible by 2");
//     } else if num % 3 == 0 {
//         println!("number is divisible by 3");
//     } else {
//         println!("number is not divisible by 3, 2");
//     }

//     match num {
//         n if n % 2 == 0 => println!("number is divisible by 2"),
//         n if n % 3 == 0 => println!("number is divisible by 3"),
//         _ => println!("number is not divisible by 3 or 2"),
//     }
// }

// if 표현식의 각 갈래의 결괏값은 같은 타입
fn main() {
    let condition = false;

    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");
}


// // if 표현식의 각 갈래의 결괏값은 같은 타입, 다른 타입이면 에러 반환 
// fn main() {
//     let condition = false;

//     let number = if condition { 5 } else { "6 "};

//     println!("The value of number is: {number}");
// }