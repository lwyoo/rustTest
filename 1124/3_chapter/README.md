
* 변수
	* 기본적으로 불변
		* 안정성과 쉬운 동시성을 활용하는 방식?
		* 코드의 한 부분이 변숫값은 변하지 않는다는 전제 하에 작동하고 코드의 다른 부분이 그 값을 바꾼다면, 앞부분의 코드는 원래 지정된 일을 못할 가능성이 생김
		* 코드를 읽고 쓸 때 값이 어디서 어떻게 바뀔지 추적할 필요가 없다는 것
* 상수
	* mut와 함께 사용할 수 없음
	* 항상 불변
	* const 키워드로 선언
	* 단어 사이에 밑줄, 모든 글자를 대문자
	* 선언된 스코프 내, 프로그램이 동작하는 전체 시간 동안 유효
* 섀도잉
	* 새 변수를 이전 변수명과 같은 이름으로 선언
	* 첫 번째 변수가 두 번째 변수에 의해 가려졌다 (shadowed) 라고 표현
	* mut 와 섀도잉의 차이
		* 실수로 let 키워드 없이 변수에 값을 재할당하려고 한다면 컴파일 타임 에러가 발생
		* let을 사용하면, 값을 변형하면서 변형이 완료된 후에는 불변으로 유지
		* let 키워드를 사용하여 새로운 변수를 만드는 것이기 때문에 같은 변수명으로 다른 타입의 값을 저장할 수 있다

* 데이터 타입
	* 러스트는 정적 타입(statically typed)의 언어이다
		* 변수나 함수의 타입이 컴파일 시점에 결정되는 언어
			* c++ 에서의 Template 도 컴파일 타임에 결정
	* 스칼라 티입  
		하나의 값을 표현한다
		* 정수  

		| 길이     | 부호 있음 (signed) | 부호 없음 (unsigned) |
		|----------|--------------------|----------------------|
		| 8-bit    | i8                 | u8                   |
		| 16-bit   | i16                | u16                  |
		| 32-bit   | i32                | u32                  |
		| 64-bit   | i64                | u64                  |
		| 128-bit  | i128               | u128                 |
		| arch     | isize              | usize                |
		* 부동 소수점
			* f32
			* f64 (default)
		* 부울린
			* bool
		* 문자
			* char
	* 복합 타입 (여러 개의 값을 하나의 단위로 묶은 타입)
		* tuple
			* 다양한 타입의 여러 값을 묶어 하나의 복합 타입으로 만드는 일반적인 방법
			* 고정된 길이
			* 괄호 안에 쉼표로 구분
		* array
			* 모든 요소는 모두 같은 타입
			* 배열은 고정된 길이
			* example
				* let a: [i32; 5] = [1, 2, 3, 4, 5];
				* let a: [3; 5] // 3값을 4개 넣는 배열
				* let a: [u32; 5] = [3; 5]; // u32 타입의 값 3을 5개 넣는 배열

* 함수
	* 표기법
		* 스네이크 케이스
	* 구문
		*  어떤 동작을 수행하고 값을 반환하지 않는 명령어
			* x = y = 6 불가
	* 표현식
		* 결괏값을 평가
		* 표현식은 종결을 나타내는 세미콜론을 쓰지 않음
		  ``` rust
			fn main() {
				let y = {
					let x = 3;
					x + 1
				};
				println!("The value of y is: {y}");
			}
* 주석
	* 
* 제어 흐름
	* 특징   
		조건은 반드시 bool 타입이어야 합니다.  
		예: if number (C/C++ 스타일)과 같은 표현은 Rust에서 컴파일 에러를 발생시킵니다.
	* if 문은 조건을 검사하고, 조건이 true일 경우 코드 블록을 실행
	
	* 표기법
		* 기본 if 문 구조
			* if 뒤에는 소괄호가 필요하지 않습니다.
			* 조건문은 반드시 bool 타입이어야 합니다. (숫자 0이나 1을 사용하는 다른 언어와는 다릅니다.)
			* 조건이 참(true)일 때 중괄호 안의 코드가 실행됩니다.
				~~~ rust
					fn main() {
						let number = 5;
						if number > 0 {
							println!("{} is positive", number);
						}
					}
				~~~
		* 특징
			* 조건은 반드시 bool 타입이어야 함.
			* 중괄호는 선택이 아니라 필수.
			* 표현식이므로 값 반환이 가능.
			* 타입 불일치는 허용되지 않음.
		* if-else 문
			* else를 사용하여 조건이 거짓(false)일 때 실행할 코드를 작성할 수 있습니다.
			~~~ rust
				fn main() {
					let number = -3;
					if number > 0 {
						println!("{} is positive", number);
					} else {
						println!("{} is not positive", number);
					}
				}
			~~~
		* if-else if 문
			~~~ rust
				fn main() {
					let number = 0;

					if number > 0 {
						println!("{} is positive", number);
					} else if number < 0 {
						println!("{} is negative", number);
					} else {
						println!("{} is zero", number);
					}
				}
			~~~
		* if 문으로 값 반환하기
			* 각 블록의 타입이 동일해야 합니다. 위 예제에서 if와 else 모두 문자열(&str)을 반환합니다.
			* 타입이 일치하지 않으면 컴파일 에러가 발생합니다.
			~~~ rust
				fn main() {
					let number = 5;

					let result = if number % 2 == 0 {
						"even"
					} else {
						"odd"
					};

					println!("The number is {}", result);
				}
			~~~
			* if let과의 차이
				* Rust에서는 if let 구문도 자주 사용됩니다. 이는 패턴 매칭과 관련된 기능으로, 일반 if 문과는 다른 목적으로 사용됩니다.
				~~~ rust
					fn main() {
						let some_value = Some(5);

						if let Some(x) = some_value {
							println!("The value is {}", x);
						} else {
							println!("No value");
						}
					}
				~~~

* 반복문
	* loop
	* while
	* for


	