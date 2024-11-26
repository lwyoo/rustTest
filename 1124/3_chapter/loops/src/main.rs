// fn main() {
//     let mut cnt = 0;
//     loop {
//         println!("again!!");
//         cnt += 1;
//         if cnt == 10 {
//             break;
//         }
//     }
// }

// // 단순 loop  테스트 프로그램 
// fn main() {
//     let mut cnt = 0;
//     let ret = loop {
//         println!("again!!");
//         cnt += 1;
//         if cnt == 10 {
//             break cnt * 2
//         }
//     };

//     println!("result : {ret}");
// }


// loop label 
// label_name: for (조건) {
//     // 루프 코드
//     if (조건) {
//         break label_name; // 특정 루프를 종료
//     }
// }

// // loop label example
// fn main() {
//     /* 
//         1. loop(counting_up) 1 회 
//             - count is 0
//             loop(loop) 1회 실행
//                 - remaining  is 10
//                 - remaining is 9
//         2. loop(counting_up) 2 회 
//             - count is 1
//             loop(loop) 1회 실행
//                 - remaining  is 10
//                 - remaining is 9
//         3. loop(counting_up) 3 회 
//             - count is 2
//             loop(loop) 1회 실행
//                 - remaining  is 10
//             - end count 2
//             - 종료

//     */
//     let mut count = 0;
//     'counting_up: loop {
//         println!("count = {count}");
//         let mut remaining = 10;

//         loop {
//             println!("remaining = {remaining}");
//             if remaining == 9 {
//                 break;
//             }
//             if count == 2 {
//                 break 'counting_up; // 해당 label 의 loop 를 멈춰라 ~
//             }
//             remaining -= 1;
//         }

//         count += 1;
//     }
//     println!("End count = {count}");
// }

fn main() {

    let mut my_value = 0;

    let mut count_value = 0;
    'my_loop: loop {
        println!("count_value: {count_value}");
        
        loop {
            // increase
            count_value = count_value + 1;
            
            println!("@@@@ count_value: {}", count_value);
            if count_value == 10 {
                break 'my_loop;
            } else {
                break;
            }
        }
        println!("update value {count_value}");
    }

}