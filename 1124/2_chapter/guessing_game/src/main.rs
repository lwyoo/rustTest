// 사용자의 입ㅕ을 받고 결과를 표시 하기 위해 사용
// 프렐루드: 기본적으로 러스트는 모든 프로그램의 스코프로 가져오는 표준 라이브러리에 정의된 아이템 집합을 가지고 있습니다. 이 집합을
// https://doc.rust-lang.org/std/prelude/index.html
use std::io;

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");
 
    /*
        let: 변수를 선언하기 위한 구문
            rust 에서 기본적으로 변수를 불변이다!!!
                let temp = 3;
            가변 변수를 선언 하기 위해서 는 mut 구문 추가 필요
                let tmu temp = 3; 
     */

    /* 
        연관 함수 
            어떤 타입에 구현된 함수
            String::new() - 비어있는 새 문자열을 생성하다
    */

    // 빈 string 인스턴스를 묶어 넣은 가변 변수를 생성한다
    let mut guess = String::new();

    /* 
        io::stdin()
            io 모듈의 연관 함수인 stdin을 이용하여 사용자의 입력을 처리한다
            터미널의 표준 입력의 핸들 (handle) 을 나타내는 타입인 std::io::Stdin의 인스턴스를 반환
        read_line()
            사용자로부터 입력받기 위해 표준 입력 핸들에서 read_line 메서드를 호출
            &mut guess를 read_line의 인수로 전달하여 사용자 입력이 어떤 문자열에 저장될 것인 알려준다
            &mut guess
                & 매모라 복사 하지 않고 참고
                mut 참조자를 가변으로 처리

    */
    io::stdin().read_line(&mut guess).expect("Failed to read line");

    /* 
        자리 표시자 (placeholder)
            어떤 변수의 값을 출력할 때라면 해당 변수 이름을 이 중괄호 안에 넣을 수 있습니다
            빈 중괄호를 형식 문자열에 위치시키고, 그 뒤에 쉼표로 구분된 표현식
                let x = 3;
                let y = 6;
                println!("x = {x}, y + 3  = {}", y +3);
    */
    println!("You guesseg: {guess}");



}