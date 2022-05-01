// 该属性用于隐藏对未使用代码的警告。
#![allow(dead_code)]

// 拥有隐式辨认值（从0开始）的enum
enum Number {
    Zero,
    One,
    Two,
}

// 拥有显式辨别值（explicit discriminator）的 enum
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn main() {
    println!("zero is {}", Number::Zero as i32);
    println!("one is {}", Number::One as i32);

    // {:06x}中的06是16进制需要保留的位数
    println!("roses are #{:06x}", Color::Red as i32);
    println!("violets are #{:06x}", Color::Green as i32);
}
