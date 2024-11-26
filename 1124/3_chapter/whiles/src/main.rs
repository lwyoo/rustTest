fn main() {
    ex_default();
    ex_while_with_array();

    ex_while_with_array2();

    ex_for_with_array();

    ex_for_with_array2();

    ex_for_with_range();
}

fn print_function_name<T>(_: T) {
    let full_type_name = std::any::type_name::<T>();
    if let Some(function_name) = full_type_name.split("::").last() {
        println!("Function name: {}()", function_name);
    }
}

fn ex_default() {
    print_function_name(ex_default);
    let mut count = 0;
    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }
    println!("end count: {}", count);
}

// 배열을 이용하여 while 문 동작 시키기
// for 반복문을 이용하여 콜랙션의 각 요소 순회하기
// 컬렉션(Collection)**은 데이터를 저장하고 관리할 수 있는 자료 구조를 의미
fn ex_while_with_array() {
    print_function_name(ex_while_with_array);
    let a = [5,4,3,2,1];
    let mut index = 0;
    
    while index < 5 {
        println!("a[{index}]: {}", a[index]);
        index += 1;
    }
    println!("end!!!!");
}

fn ex_while_with_array2() {
    print_function_name(ex_while_with_array2);
    // let a = [5,4,3,2,1];
    let a: [u32; 10] = [5,4,3,2,1, 2, 3, 4, 5, 0];
    let mut index = 0;
    
    while index < 10 {
        println!("a[{index}]: {}", a[index]);
        index += 1;
    }
    println!("end!!!!");
}


fn ex_for_with_array() {
    print_function_name(ex_for_with_array);
    // let a = [5,4,3,2,1];
    let a: [u32; 10] = [5,4,3,2,1, 2, 3, 4, 5, 0];
    
    for temp in a {
        println!("temp : {temp}");
    }
    println!("end!!!!");
}


fn ex_for_with_array2() {
    print_function_name(ex_for_with_array2);
    // let a = [5,4,3,2,1];
    let a: [u32; 10] = [5,4,3,2,1, 2, 3, 4, 5, 0];
    
    /* 
        a.iter()
            a 배열의 이터레이터를 생성합니다. 
            이 이터레이터는 배열의 각 요소를 차례대로 반환합니다.
        enumerate()
            이터레이터에서 각 요소와 함께 해당 요소의 인덱스를 포함한 튜플을 반환합니다.
    
    */
    for (index, value) in a.iter().enumerate() {
        println!("a[{index}] : {value}");
    }
    println!("end!!!!");
}


fn ex_for_with_range() {
    for number in (1..=4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}