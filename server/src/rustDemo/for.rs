fn main() {
    // 1.借用集合中的一个元素
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustaceam among us"),
            _ => println!("Hello {}", name),
        }
    }

    // 2.消耗集合，数据本身被提供
    let names = vec!["Bob", "Frank", "Ferris"];
    for name in names.into_iter() {
        match name {
            "Ferris" => println!("There is a rustacean among us!"),
            _ => println!("Hello {}", name),
        }
    }
    // 以下这句会报错
    // println!("end {:?}", names);

    // 3.可变地借用集合中地每个元素，从而允许集合被就地修改
    let mut names = vec!["Bob", "Frank", "Ferris"];
    for name in names.iter_mut() {
        *name = match name {
            &mut "Ferris" => "There is a rustacean among us!",
            _ => "Hello",
        }
    }
    println!("names:{:?}", names);
}
