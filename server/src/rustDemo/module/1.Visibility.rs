// 定义一个名为my_mod的模块
mod my_mod {
    // 模块中的项默认具有私有的可见性
    fn private_function() {
        println!("I am my_mod::private_function")
    }
    // pub共有的可见性
    pub fn function() {
        println!("I am my_mod::function")
    }
    // 同一模块间，项之间的嵌套访问
    pub fn indirect_access() {
        println!("I am indirect_access");
        private_function()
    }
    // 模块之中的模块，pub使mod变为公有
    pub mod nested {
        fn private_function() {
            println!("I am my_mod::nested::private_function");
        }
        pub fn function() {
            println!("I am my_mod::nested::function")
        }

        // pub(in path) 语法定义的函数只在给定的路径中可见，且path必须是父模块或祖先模块
        pub(in crate::my_mod) fn public_function_in_my_mod(){
            println!("I am my_mod::nested::public_function_in_my_mod")
        }

        // pub(self) 定义的函数只在当前模块中可见
        pub(self) fn public_function_in_nested() {
            println!("I am my_mod::nested::public_function_in_nested")
        }
        
        // pub(super) 定义的函数只在父模块中可见
        pub(super) fn public_function_in_super_mod() {
            println!("I am my_mod::nested::public_function_in_super_mod")
        }
    }
    pub fn call_public_function_in_my_mod() {
        println!("I am my_mod::call_public_function_in_my_mod")
        nested::public_function_in_my_mod();
        nested::public_function_in_super_mod();
    }
    // 只在当前crate中可见
    pub (crate) fn public_function_in_create(){
        println!("I am  my_mod::public_function_in_create")
    }

    // 嵌套模块的可见性遵循相同的原则,默认private
    mod private_nested{
        pub fn function(){
            println!("I am my_mod::private_nested::function");
        }
    }

}
fn function(){
    println!("I am function")
}
fn main() {
    // 不同模块间项的名字可以相同
    function();
    my_mod::function();

    // 公有项，直接访问
    my_mod::indirect_access();
    my_mod::nested::function();
    my_mod::call_public_function_in_my_mod();

    // pub(crate) 项可以在同一个crate中的任何地方访问
    my_mod::public_function_in_create();

    // 报错，public_function_in_my_mod只能在my_mod模块中访问
    // my_mod::nested::public_function_in_my_mod();

    // 报错！`private_function` 是私有的
    // my_mod::private_function();

    // Error! `private_nested` is a private module
    // my_mod::private_nested::function();
}
