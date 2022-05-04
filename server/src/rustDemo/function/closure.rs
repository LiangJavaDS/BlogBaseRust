fn main() {
    // 使用函数实现自增
    fn function(i: i32) -> i32 {
        i + 1
    }
    /*
    1.闭包是匿名的，需要保存到变量上调用
    2.声明闭包时，用||代替()将输入的参数括起来
    3.函数节点符{}，对于单个表达式是可选的，其他情况必须加上
    4.有能力捕获外部环境的变量
    */
    let closure_annotated = |i: i32| -> i32 { i + 1 };
    let closure_inferred = |i| i + 1;
    let i = 1;
    println!("f:{}", function(i));
    println!("c1:{}", closure_annotated(i));
    println!("c2:{}", closure_inferred(i));
    // 没有参数的闭包，返回一个自动推导的类型i32
    let one = || 1;
    println!("closure returning one: {}", one());

    use std::mem;
    let color = String::from("green");
    /*
    此闭包打印color，它会立即借用（通过引用，'&'）'color'并将该借用和闭包本身存储到print变量中。
    color会一直保持被借用状态知道print离开作用域
    */
    let print = || println!("'color':{}", color);
    print();

    // color可再次被不可变借用，闭包只持有一个指向color的不可变引用。
    let _reborrow = &color;
    print();

    // 在最后使用print后，移动或借用都是允许的
    let _color_moved = color;
    let mut count = 0;
    /*
    1.这个闭包使count增加。需要得到&mut count或者count本身。但&mut count地要求没那么严格，闭包采取&mut count方式
    2.inc前加mut，闭包里存储着一个&mut变量。调用闭包时，变量的变化意味着闭包内部
    发生了变化。因此闭包需要是可变的
    */
    let mut inc = || {
        count += 1;
        println!("count = {}", count)
    };
    inc();
    // 这行之后调用inc闭包，所以仍然可变借用count
    // let _reborrow = &count;  // error
    inc();

    // 闭包不再借用&mut count，因此可以正确地重新借用
    let _count_reborrowed = &mut count;

    // 不可复制类型
    let movable = Box::new(3);
    /*
    mem::drop要求T类型本身，所以闭包将会捕获变量地值，所以闭包将会捕获变量的值。
    这种情况下，可复制类型将会复制给闭包，从而原始值不受影响。不可复制类型必须移动到闭包中。
    因此上一句的movable变量立即移动到了闭包中。
    */
    let consume = || {
        println!("`movable`:{:?}", movable);
        mem::drop(movable);
    };
    // consume消耗了该变量，所以该闭包只能调用一次
    consume();
    // consume();

    // Vec再语义上不可复制
    let haystack = vec![1, 2, 3];
    // 在|前使用move会强制闭包取得被捕获变量的所有权
    let contains = move |needle| haystack.contains(needle);
    println!("{}", contains(&1));
    println!("{}", contains(&4));
    // 下面一行将会报错，借用检查不允许在变量被移走之后继续使用它
    // println!("Threr're {} elements in vec", haystack.len());
    // 闭包签名删除move关键字，导致闭包以不可变的形式借用haystack,所以haystack仍然可用
}
