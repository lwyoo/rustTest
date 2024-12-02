// TODO: 공백문자로 구분된 단어들의 문자열을 입력받아서 해당 문자열의 첫 번째 단어를 반환하는 함수
fn main() {
    println!("Hello, world!");
    let my_string = String::from("Hello, world!");
    let str_len = first_word(&my_string);
    println!("str_len: {str_len}");

    let str_len = first_word(&my_string[7..13]);
    println!("str_len: {str_len}");

    let str_len = first_word(&my_string[..]);
    println!("str_len: {str_len}");

    let my_string_literal = "hello world";
    
    let str_len = first_word(&my_string_literal);
    println!("str_len: {str_len}");

    // 그 외 slice
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    for item in slice {
        println!("item: {item}");
    }
    assert_eq!(slice, &[2, 3]);
}

/* 
    &String 을 &str로 변환이 가능한 이유
        String 이 내부적으로 str 데이터를 소유하고 있기 때문
        Rust에서는 String이 Deref 트레이트를 구현하여, 자동으로 &String을 &str로 변환할 수 있도록 설계되어 있습니다.
*/
fn first_word(s: &str) -> &str {
    // 공백문자를 가리키는 단어 끝부분의 인덱스 반환

    // String을 as_bytes()를 이용 하여 바이트 배열로 변환 
    let bytes = s.as_bytes();

    // iter 메서드를 이용하여 컬렉션의 각 요소 반환
    // enumerate() iter의 각 결괏값을 튜플로 감싸 반환 한다, 첫번쨰 요소 인덱스, 두번째 요소가 해당 요소의 참조자
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}