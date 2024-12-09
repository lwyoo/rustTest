* 구조체  
  여러 값을 묶고 이름을 지어서 의미 있는 묶음을 정의하는 데에 사용
  객체의 데이터 속성과 비슷
  * 구조체와 튜플 비교 && 어떤 경우 사용 하는지
    * 구조체는 여러개의 연관된 값을 가질 수 있다 (튜플과 동일)
    * 구조체의 구성 요소들은 각각 다른 타입 (튜플과 동일)
    * 구조체는 각각의 구성 요소에 이름을 붙일 수 있습니다 (튜플 x)
    * 특정 요소에 접근할 때 순서에 의존할 필요도 사라지게 됨 (튜플 x)
  * 구조체 정의 및 생성 방법
    * struct 키워드와 해당 구조체에 지어줄 이름을 입력
    * 중괄호 안에서는 필드 (field) 라고 부르는 각 구성 요소의 이름 및 타입을 정의
    ``` rust
    // 구조체 정의
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }

    // 구조체 인스턴스 생성
    fn main() {
        // 가변성은 해당 인스턴스 전체가 지니게 됩
        // 일부 필드만 가변으로 만들 수는 없음
        let mut user1 = User {
            active: true,
            username: String::from("someusername123"),
            email: String::from("someone@example.com"),
            sign_in_count: 1,
        };
    // 구조체 데이터 변경
        user1.email = String::from("anotheremail@example.com");
    }

    ```
  * 메서드
  * 연관 함수