// 移动到main.rs中执行
/*
此声明会查找名为 my.rs 或 my/mod.rs 的文件，
并将文件内容放到此作用域中一个名为my的模块里
*/
mod my;

fn function() {
    println!("called `function()`");
}

fn main() {
    my::function();
    function();
    my::indirect_access();
    my::nested::function();
}
