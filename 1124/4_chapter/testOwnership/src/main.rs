// fn main() {
// 	// ownership 관련 테스트 
//     println!("Hello, world!");
// 	let _s = "literal string";

// 	let _s = String::from("test String");
// 	println!("s: {}", _s);

// 	let mut ss = _s.clone();
// 	ss.push_str(", aaaa");

// 	println!("s: {}", _s);
// 	println!("ss: {}", ss);

// 	let out_string = test(ss);
// 	println!("out_string: {}", out_string);
// 	println!("ss: {}", ss);
// }

// fn test(a_string: String) -> String {
// 	println!("inputValue: {}", a_string);

// 	let some_string = String::from("some_string");

// 	some_string
// } 


fn main() {
	println!("\tcase1. 스택에 저장된 값은 복사가 일어남 - copy trait");
	let _s = "리터럴 문자열";
	let _ss = _s;
	println!("_s: {_s}");
	println!("_ss: {_ss}");

	println!("\tcase2. 힙에 저장된 값은 이동이 일어남 - move trait");
	let mut _s = String::from("heap Value test");
	_s.push_str(", udpate~!!!");
	println!("_s: {_s}");


	println!("\tcase3. string 복사(heap) 관련 ownership 이동");
	let _s = String::from("heal memory string");
	let _ss = _s;
	// move 로 인해 해당 값 없음
	// println!("_s: {_s}"); 
	println!("_ss: {_ss}");

	println!("\tcase4. clone 으로 복사 하기");
	let _s = String::from("heal memory string");
	let _ss = _s.clone();
	println!("_s: {_s}"); 
	println!("_ss: {_ss}");

	input_arg(1);
	let out_value = output_arg();
	println!("out_value: {out_value}" );

	let out_value = input_output_arg(10);
	println!("out_value: {out_value}" );

	let s = String::from("abcd");
	let (ss, len) = calculate_length(s);

	println!("ss: {ss}");
	println!("length: {len}");

}

fn input_arg(input_value:i32) {
	println!("input value : {input_value}");
}

fn output_arg() -> i32 {
	let out_value = 32;
	out_value
}

fn input_output_arg(intput_arg:i32) -> i32 {
	let out_value = intput_arg * 3;
	out_value
}

fn calculate_length(s: String) -> (String, usize) {
	// move 했던값을 다시 반환 한다, 불필요한 반한이라는뎅~
	let length = s.len();
	(s, length)
}