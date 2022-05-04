mod my {
    // 一个公有的结构，带有一个公有的字段（泛型T）
    pub struct OpenBox<T> {
        pub contents: T,
    }
    // 一个公有的结构体，带一个私有的字段（泛型T）
    #[derive(Debug)]
    pub struct ClosedBox<T> {
        contents: T,
    }
    impl<T> ClosedBox<T> {
        // 公有的构造器方法
        pub fn new(contents: T) -> ClosedBox<T> {
            ClosedBox { contents: contents }
        }
    }
}
fn main() {
    // 带有公有字段的公有结构体，可以像平常一样构造
    let open_box = my::OpenBox {
        contents: "public information",
    };
    // 并且它们的字段可以正常访问到
    println!("The open box contains: {} ", open_box.contents);

    // 带有私有字段的公有结构体不能使用字段名来构造
    // Error: field `contents` of struct `my::ClosedBox` is private
    // let closed_box = my::ClosedBox {
    //     contents: "classified information",
    // };

    // 带有私有字段的结构体可以使用公有的构造器来创建
    let closed_box = my::ClosedBox::new("classified information");
    println!("The closed box contains: {:?}", closed_box);
}
