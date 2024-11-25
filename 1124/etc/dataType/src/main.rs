// use std::io;

// fn main() {
//     println!("Hello, world!");
//     // 데이터를 입력 받고 해당 데이터 타입이 무엇인지 확인 하는거 만들기!!!!
//     println!("데이터를 입력해 봐라 !!!");

//     // 입력 받을 변수 선언
//     let mut input = String::new();

//     // 입력 받기
//     io::stdin().read_line(&mut input).expect("[Error] Failed to read line");

//     // 줄바꿈 제거
//     let input = input.trim();

//     // 타입 확인 하기
//     if let Ok(value) = input.parse::<i32>() {
//         println!("입력한 값 {} 는 (i32) 입니다", value);
//     } else if let Ok(value) = input.parse::<f64>() {
//         println!("입력한 값 {} 는 (f32) 입니다", value);
//     } else {

//         if let Some(value) = parse_bool(input) {
//             println!("입력한 값 {} 는 (bool) 입니다", input);
//         } else {
//             println!("입력한 값 {} 는 (문자열) 입니다", input);
//         }
//     }

//     // 입력값 출력
//     println!("you input {}", input);
// }

// /* 
//     입력: &str 타입을 입력
//     반환: Option<bool> 타입 반환
//         Option 값이 있을 수도 있고, 없을 수도 있는 경우를 표현
//             값이 있는 경우 Some(value)
//             값이 없는 경우 Nome
// */
// fn parse_bool(input: &str) -> Option<bool> {
//     match input {
//         "true" => Some(true),
//         "false" => Some(false),
//         _ => None,
//     }
// }

// Option에 값을 넣고 해당 값이 들어있는지 확인 하는 방법 1
// fn main() {
//     let maybe_value = Some(5);
//     match maybe_value {
//         Some(v) => println!("값이 있습니다: {}", v),
//         None => println!("값이 없습니다."),
//     }
// }

// // Option에 값을 넣고 해당 값이 들어있는지 확인 하는 방법 1
// fn main() {
//     // let maybe_value = Some(5);
//     let maybe_value: Option<i32> = None;
//     if let Some(v) = maybe_value {
//         println!("값이 있습니다: {}", v);
//     } else {
//         println!("값이 없습니다.");
//     }
// }


//  // tuple 고정되 길이, 복합 타입, 표기시 ()
// fn main() {
//     // tuple 선언 하고 바인딩 하기 
//     println!("tuple test");

//     let tup = (10, "myStr", 'a');
//     let (x, y, z) = tup;
//     println!("x: {x}, y: {y}, z: {z}");

//     let tup: (i32, i32, &str) = (10, 20, "aaaa");
//     let (x, y, z) = tup;
//     println!("x: {x}, y: {y}, z: {z}");
// }

// fn main() {
//     // tuple 을 선언 하고 이후 인덱스로 접근 하기 
//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;
//     let six_point_four = x.1;
//     let one = x.2;
//     // 잘못된 인덱스 접근 
//     // let onexx = x.3;

//     println!("five_hundred: {}", five_hundred);
//     println!("six_point_four: {}", six_point_four);
//     println!("one: {}", one);
//     // println!("onexx: {}", onexx);
// }


// list: 고정된 길이, 단일 타입, 표기시 [], stack에 데이터 저장시 유용함
fn main() {
    let a = [1,2,3,2,1];
    println!("a[0]: {}", a[0]);
    println!("a[1]: {}", a[1]);
    println!("a[2]: {}", a[2]);
    println!("a[3]: {}", a[3]);
    println!("a[4]: {}", a[4]);

    let b: [i32; 5] = a;
    println!("b[0]: {}", b[0]);
    println!("b[1]: {}", b[1]);
    println!("b[2]: {}", b[2]);
    println!("b[3]: {}", b[3]);
    println!("b[4]: {}", b[4]);

    // let c: [i32; 5] = 0..=5;
    /* 
        여기서 0..=4는 0부터 4까지의 값을 생성하고, 
        .collect::<Vec<i32>>()는 그 값을 Vec<i32>로 수집합니다. 
        그런 다음 .try_into()를 사용하여 벡터를 고정 크기 배열로 변환합니다. 
        
        만약 변환이 실패하면 unwrap() 때문에 패닉이 발생합니다. 
        더 안전하게 처리하려면 unwrap() 대신 에러 처리를 사용할 수 있습니다.
    */
    let c: [i32; 5] = (0..=4).collect::<Vec<i32>>().try_into().unwrap();
    println!("c[0]: {}", c[0]);
    println!("c[1]: {}", c[1]);
    println!("c[2]: {}", c[2]);
    println!("c[3]: {}", c[3]);
    println!("c[4]: {}", c[4]);
    
}