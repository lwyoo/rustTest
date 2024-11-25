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


fn main() {
    let mut cnt = 0;
    let ret = loop {
        println!("again!!");
        cnt += 1;
        if cnt == 10 {
            break cnt * 2
        }
    };

    println!("result : {ret}");
}
