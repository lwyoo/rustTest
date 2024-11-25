// fn main() {
//     println!("Hello, world!");
// 	let x = 5; 
// 	println!("x: {x}");

// 	x = 10; 
// 	println!("x: {x}");
// }


// fn main() {

// 	println!("x = 5");
// 	let x = 5;
// 	println!("x: {x}");

// 	println!("x = x + 1");
// 	let x = x + 1;
// 	println!("x: {x}");

// 	{
// 		println!("x = x + 1");
// 		let x = x + 1;
// 		println!("x: {x}");
// 	}

// 	println!("x: {x}");
// }


// fn main() {

// 	println!("x = 5");
// 	let mut x = 5;
// 	println!("x: {x}");

// 	println!("x = x + 1");
// 	x = x + 1;
// 	println!("x: {x}");

// 	{
// 		println!("x = x + 1");
// 		let x = x + 1;
// 		println!("x: {x}");
// 	}

// 	println!("x: {x}");
// }

// fn main() {

// 	let a = 42; // 기본적으로 i32 타입
// 	println!("a: {a}");

// 	let a: i8 = 42i8;   // 8-bit 부호 있는 정수
// 	let b: i16 = 100i16; // 16-bit 부호 있는 정수
// 	let c: i64 = 1000i64; // 64-bit 부호 있는 정수
// 	println!("a: {a}");
// 	println!("b: {b}");
// 	println!("c: {c}");

// 	let a: isize = 42;  // 아키텍처에 맞는 부호 있는 정수
// 	let b: usize = 100; // 아키텍처에 맞는 부호 없는 정수
// 	println!("a: {a}");
// 	println!("b: {b}");

// 	let a = 42_222; // 기본적으로 i32 타입
// 	println!("a: {a}");

// 	let a = 0xff;
// 	println!("a: {a}");

// 	let a = 0o77;
// 	println!("a: {a}");

// 	let a = 0b1111_1111;
// 	println!("a: {a}");

// 	let a = b'a';
// 	println!("a: {a}");

// 	let a:u8 = 257;
// 	println!("a: {a}");
// }

	/* test overflow 
		- debug mode 에서는 panicked 발생
		- release 모드에서 wrap around 발생
	*/
// fn main() {
//     let a: u8 = 255;
//     let b = a + 1;  // 256은 u8의 범위를 초과하지만, wrap around가 발생하여 0이 됩니다.
//     println!("a: {a}, b: {b}");
// }

use std::io;

fn main() {
    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}