// fn main() {
//     let mut s = String::from("hello");

//     let r1 = &s; // 문제없음
//     let r2 = &s; // 문제없음
//     println!("{} and {}", r1, r2);
//     // 이 지점 이후로 변수 r1과 r2는 사용되지 않습니다

//     let r3 = &mut s; // 문제없음
//     println!("{}", r3);
// }

let mut s = String::from("hello");

fn main() {
    let r1 = &mut s; // 문제없음
    println!("r1: {r1}");
    let r2 = &mut s; // 문제없음
    println!("r2: {r2}");
}
