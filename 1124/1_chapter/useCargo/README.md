* creat
    * cargo 를 이용한 프로젝트 생성
        * cargo new [PorjectName]
    * vsc 추가
        * cargo new [PorjectName] --vcs=git

* build
    * 빌드 명령어 (debug build)
        * cargo build
        * cargo b
        * cargo b --relase
    * 생성 파일
        * target
            * cargo.lock  
            프로젝트에서 사용하는 의존성의 정확한 버전을 자동으로 기록해 두는 파일이니 여러분이 직접 수정할 필요는 없습니다. 물론 이번 프로젝트는 의존성을 갖지 않으므로 현재는 파일에 특별한 내용이 없습니다.
* 실행
    * cargo run 을 이용해 실행 
    * cargo r
    * cargo r --relase
*  실행 파일은 생성하지 않고 작성한 소스가 문제없이 컴파일되는지만 빠르게 확인
    * cargo check
    * cargo c
    * cargo c --release