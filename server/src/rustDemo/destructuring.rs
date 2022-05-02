#[allow(dead_code)]
// 2.解构枚举
enum Color {
    Red,
    Blue,
    Green,
    // 这些则把 `u32` 元组赋予不同的名字，以色彩模型命名。
    RGB(u32, u32, u32),
    HSV(u32, u32, u32),
    HSL(u32, u32, u32),
    CMY(u32, u32, u32),
    CMYK(u32, u32, u32, u32),
}
fn main() {
    // 1.解构元组
    let triple = (1, -2, 3);
    println!("Tell me about{:?}", triple);
    match triple {
        (0, y, z) => println!("First is `0`, `y` is {:?}, and `z` is {:?}", y, z),
        (1, ..) => println!("First is `1` and the rest doesn't matter"),
        _ => println!("It doesn't matter what they are"),
    }
    // 2.解构枚举
    let color = Color::RGB(122, 17, 40);
    println!("What color is it?");
    match color {
        Color::Red => println!("The color is Red!"),
        Color::Blue => println!("The color is Blue!"),
        Color::Green => println!("The color is Green!"),
        Color::RGB(r, g, b) => println!("Red:{},green:{},and blue:{}!", r, g, b),
        Color::HSV(h, s, v) => println!("Hue: {}, saturation: {}, value: {}!", h, s, v),
        Color::HSL(h, s, l) => println!("Hue: {}, saturation: {}, lightness: {}!", h, s, l),
        Color::CMY(c, m, y) => println!("Cyan: {}, magenta: {}, yellow: {}!", c, m, y),
        Color::CMYK(c, m, y, k) => println!(
            "Cyan: {}, magenta: {}, yellow: {}, key (black): {}!",
            c, m, y, k
        ),
    }
    /*
    3.解构指针
    3.1 解引用使用*
    3.2 解构使用&、ref、ref mut
    */

    // 获得一个 `i32` 类型的引用。`&` 表示取引用。
    let reference = &4;
    // `&val` 这个模式去匹配 `reference` =》 `&i32` 匹配 `&val =》 去掉匹配&,'i32' 赋值给 'val'
    match reference {
        &val => println!("Got a value via destructuring:{:?}", val),
    }
    // 匹配前解引用
    match *reference {
        val => println!("Got a value via dereferencing:{:?}", val),
    }

    let ref _is_a_reference = 3;
    let value = 5;
    let mut mut_value = 6;
    // 使用 `ref` 关键字来创建引用。
    match value {
        ref r => println!("Got a reference to a value:{:?}", r),
    }
    match mut_value {
        ref mut m => {
            // * 获取'mut_value'的引用，先解引用，才能改变它的值
            *m += 10;
            println!("We added 10. 'mut_value':{:?}", m);
        }
    }
    // 4.解构结构体
    struct Foo {
        x: (u32, u32),
        y: u32,
    }
    let foo = Foo { x: (1, 2), y: 3 };
    let Foo { x: (a, b), y } = foo;
    println!("a = {}, b = {},  y = {} ", a, b, y);
    // 解构结构体并重命名变量，成员顺序不重要
    let Foo { y: i, x: j } = foo;
    println!("i = {:?}, j = {:?}", i, j);
    let Foo { y, x } = foo;
    println!("y = {:?}, x = {:?}", y, x);
    // 忽略某些变量
    let Foo { y, .. } = foo;
    println!("y={}", y);
}
