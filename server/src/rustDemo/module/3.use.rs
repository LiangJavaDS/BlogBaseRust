// use将一个完整的路径绑定到一个新的名字，从而更容易访问
use deeply::nested::function as other_function;

fn function() {
    println!("function")
}

mod deeply {
    pub mod nested {
        pub fn function() {
            println!("deeply::nested::function")
        }
    }
}

fn main() {
    // 更容易访问 deeply::nested::function
    other_function();
    println!("Entering block");
    {
        // 此function()将遮蔽外部的同名函数
        use deeply::nested::function;
        function();
        println!("Leaving block");
    }
    function();
}
