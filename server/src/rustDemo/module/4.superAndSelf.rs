fn function() {
    println!("function");
}

mod cool {
    pub fn function() {
        println!("cool::function")
    }
}

mod my {
    fn function() {
        println!("my::function")
    }
    mod cool {
        pub fn function() {
            println!("my::cool::function")
        }
    }
    pub fn indirect_call() {
        println!("my::indirect_call()");
        // self代表当前的模块作用域，在这个例子中是my
        // 以下两句等价
        self::function();
        function();

        self::cool::function();
        // super表示父作用域，my的上一层
        super::function();
        // crate作用域内绑定"cool::function"
        // 此例，crate作用域是最外面的作用域
        {
            use crate::cool::function as root_function;
            root_function();
        }
    }
}
fn main() {
    my::indirect_call()
}
