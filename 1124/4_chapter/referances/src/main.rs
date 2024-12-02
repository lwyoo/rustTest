fn main() {
    let ref_str = String::from("go to hell!!");

    let len = test_referance(&ref_str);
    println!("str value: [{ref_str}], size: [{len}]");

    let mut ref_str = String::from("go to hell!!");
    let len = test_mut_referance(&mut ref_str);
    println!("str value: [{ref_str}], size: [{len}]");

    let mut s = String::from("hello");

}

fn test_referance(ref_str:&String) -> usize {
    let length = ref_str.len();
    // 참조한 값을 변경하는 경우?? 가능 한가? 불가능 합니다 ~
    // ref_str.push_str(", ??????");
    length
}


fn test_mut_referance(ref_str:&mut String) -> usize {
    // 가변 참조자
    let length = ref_str.len();
    ref_str.push_str(", ??????");
    length
}
