fn main() {
    println!("0..5 에 대한 처리 - 시작부터 끝 전 범위 까지");
    for i in 0..5 {
        println!("{}", i);
    }

    println!("0..=5 에 대한 처리 - 시작부터 끝까지의 범위 (끝값 포함)");
    for i in 0..=5 {
        println!("{}", i);
    }

    println!("0..=5 에 대한 처리 - 시작부터 끝까지의 범위 (끝값 포함)");
    for i in 0..=5 {
        println!("{}", i);
    }

    println!("범위: 시작 값 포함, 끝 값 미포함.");
    let range = 1..4; // 1, 2, 3
    println!("{:?}", range.contains(&3)); // true
    println!("{:?}", range.contains(&4)); // false
    
    println!("범위: 시작 값 포함, 끝 값 포함.");
    let range = 1..=4; // 1, 2, 3, 4
    println!("{:?}", range.contains(&4)); // true

    println!("범위: 시작과 끝을 생략하여 범위를 표현.");
    let array = [10, 20, 30, 40, 50];
    // 끝까지
    println!("{:?}", &array[2..]); // [30, 40, 50]
    // 처음부터
    println!("{:?}", &array[..3]); // [10, 20, 30]
    // 전체
    println!("{:?}", &array[..]);  // [10, 20, 30, 40, 50]

    println!("패턴 매칭에서의 범위 사용");
    match 42 {
        0..=10 => println!("0에서 10 사이"),
        11..=50 => println!("11에서 50 사이"), // 여기로 매칭됨
        _ => println!("그 외"),
    }
    
    println!("Iterator와 함께 사용");
    let sum: i32 = (1..=5).sum(); // 1 + 2 + 3 + 4 + 5 = 15
    println!("{}", sum);

    println!("무한 범위 - Rust에서는 무한 범위를 생성하는 방법");
    for i in 0.. {
        if i > 10 {
            break;
        }
        println!("{}", i); // 0부터 10까지 출력
    }


}
