#![allow(dead_code)]

enum WebEvent {
    // 一个enum可以是单元结构体
    PageLoad,
    PageUnload,
    // 或一个元组结构体
    KeyPress(char),
    Paste(String),
    // 或一个普通的结构体
    Click { x: i64, y: i64 },
}

fn inspect(event: WebEvent) {
    match event {
        WebEvent::PageLoad => println!("page loaded"),
        WebEvent::PageUnload => println!("page unload"),
        // 从 `enum` 里解构出 `c`。
        WebEvent::KeyPress(c) => println!("pressed '{}'.", c),
        WebEvent::Paste(s) => println!("pasted \"{}\".", s),
        // 把 `Click` 解构给 `x` and `y`。
        WebEvent::Click { x, y } => {
            println!("clicked at x={}, y={}.", x, y);
        }
    }
}

enum AddOrSub {
    Add,
    Subtract,
}

impl AddOrSub {
    fn run(&self, x: i32, y: i32) -> i32 {
        match self {
            Self::Add => x + y,
            Self::Subtract => x - y,
        }
    }
}

fn main() {
    let load = WebEvent::PageLoad;
    let unload = WebEvent::PageUnload;
    let pressed = WebEvent::KeyPress('x');
    // 从一个字符串切片中创建一个具有所有权的'String'。
    let pasted = WebEvent::Paste("my text".to_owned());
    let click = WebEvent::Click { x: 20, y: 80 };

    inspect(load);
    inspect(unload);
    inspect(pressed);
    inspect(pasted);
    inspect(click);

    println!("Add = {}", AddOrSub::Add.run(10, 5));
    println!("Subtract = {}", AddOrSub::Subtract.run(10, 5));
}
