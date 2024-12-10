

#[derive(Debug)]
struct Rec {
    width: i32,
    height: i32
}

impl Rec {
    fn area(&self)-> i32 {
        self.width * self.height
    }
    fn area2(&self) -> i32 {
        2 * self.width * self.height
    }
    fn get_width(&self) -> i32 {
        self.width
    }
    fn get_height(&self) -> i32 {
        self.height
    }
    fn set_width(&mut self, value: i32) {
        self.width = value;
    }

    fn can_hold(&self, other: &Rec) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: i32) -> Rec {
        Rec{width: size, height: size}
    }
}
fn main() {
    println!("Hello, world!");
    let area = rectangle(100, 15);
    println!("pixel size(arg1, arg2): {area}");

    let tup = (100, 300);
    let area = rectangle_with_tuple(tup);
    println!("pixel size(tutple): {area}");


    let mut  r = Rec {
        width: 100,
        height: 200
    };

    let area = rectangle_with_struct(&r);
    println!("pixel size(struct): {area}");
    

    println!("struct r darta: {:?}", r);
    println!("struct r darta: {:#?}", r);

    println!("impl call {}", r.area());
    println!("impl call {}", r.area2());

    println!("get width: {}", r.get_width());
    println!("get height: {}", r.get_height());

    r.set_width(10);

    println!("struct r darta: {:?}", r);
    println!("get height: {}", (&r).get_height());

    let temp = Rec{width: 1, height: 1};
    println!("can_hold: {}", r.can_hold(&temp));

    let sq = Rec::square(100);
    println!("연관 함수: {}", sq.area());

}

fn rectangle(width: i32, height: i32) -> i32 {
    width * height
}

fn rectangle_with_tuple(tup:(i32, i32)) -> i32 {
    tup.0 * tup.1
}

fn rectangle_with_struct(struct_value: &Rec) -> i32 {
    struct_value.width * struct_value.height
}