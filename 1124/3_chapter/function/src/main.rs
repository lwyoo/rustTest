// fn main() {
//     println!("Hello, world!");
//     let mut value = 19;
//     myTestFun();
//     value += 1;
//     myTestFunWithArg(value);

//     let tup = (1,2);
//     myTestFunWithTuple(tup);

//     let ret = myTestFunRet();
//     println!("ret {}", ret);

//     let result = myTestFunWithArgRet(5);
//     println!("Returned value: {result}");
// }

// fn myTestFun() {
//     println!("myTestFun()");
// }

// fn myTestFunWithArg(x: i32) {
//     println!("myTestFunWithArg({x})");
// }

// fn myTestFunWithArgRet(x: i32) -> &'static str {
//     println!("myTestFunWithArg({x})");
//     "x * 100"  // 리터럴 값 반환
// }
// // fn myTestFunWithArgRet(x: i32) -> &str {
// //     println!("myTestFunWithArg({x})");
// //     "x * 100"
// // }

// fn myTestFunWithTuple(x: (i32, u32)) {
//     println!("myTestFunWithArg({})", x.0);
//     println!("myTestFunWithArg({})", x.1);
// }

// fn myTestFunRet() -> i32 {
//     println!("myTestFun()");
//     32
// }
fn main() {
    let x = plus_one(5);

    println!("The value of x is: {x}");
}

fn plus_one(x: i32) -> i32 {
    x + 1
}