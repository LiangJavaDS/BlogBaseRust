// 实现
// 具体类型S
struct S;
// 泛型GenericVal
struct GenericVal<T>(T);

impl GenericVal<f32> {}
impl GenericVal<S> {}
// <T>必须在类型之前写出来，使类型T代表泛型
impl<T> GenericVal<T> {}

struct Val {
    val: f64,
}

struct Genval<T> {
    gen_val: T,
}

impl Val {
    fn value(&self) -> &f64 {
        &self.val
    }
}

impl<T> Genval<T> {
    fn value(&self) -> &T {
        &self.gen_val
    }
}

fn main() {
    let x = Val { val: 3.0 };
    let y = Genval { gen_val: 3i32 };
    println!("{},{}", x.value(), y.value());
}
