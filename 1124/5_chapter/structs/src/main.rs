struct User {
    name: String,
    active: bool,
    sig_count: u64
}

fn main() {
    println!("Hello, world!");
    let mut me = User {
        name: String::from("lee do gyeom"),
        active: true,
        sig_count: 100
    };

    println!("me.name: {}", me.name);
    println!("me.active: {}", me.active);
    println!("me.sig_count: {}", me.sig_count);

    me.name = String::from("New Name");
    println!("me.name: {}", me.name);

    let temp = build_user(String::from("name11"), false, 100);
    println!("temp.name: {}", temp.name);
}

fn build_user(name: String, active: bool, sig: u64) -> User {
    User {
        // 축약법, 인자값과 구조체의 값이 같으면 
        name,
        active,
        sig_count: sig
    }
}