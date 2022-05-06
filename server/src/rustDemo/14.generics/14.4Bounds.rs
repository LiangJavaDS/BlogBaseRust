// 约束

use std::fmt::Debug;
// trait
trait HasArea {
    fn area(&self) -> f64;
}
// 结构体
#[derive(Debug)]
struct Rectangle {
    length: f64,
    height: f64,
}

// 实现
impl HasArea for Rectangle {
    fn area(&self) -> f64 {
        self.length * self.height
    }
}

// 指定参数要实现的trait
fn print_debug<T: Debug>(t: &T) {
    println!("{:?}", t);
}

fn main() {
    let r1 = Rectangle {
        length: 3.0,
        height: 4.0,
    };
    print_debug(&r1);
    println!("Area:{}", r1.area());
}
