// 函数
struct A;
struct S(A);
struct SGen<T>(T);

// 函数泛型
fn reg_fn(_s: S) {}

/*
    接收SGen<A>类型的参数，SGen<>显示地接收了类型参数A，
    且在gen_spec_t中，A没有被用作泛型类型参数，所以函数不是泛型
*/
fn gen_spec_t(_s: SGen<A>) {}
fn gen_spec_i32(_s: SGen<i32>) {}
/*
    接收SGen<T>类型地参数_s
    因为SGen<T>中有T，所以这个函数是关于T的泛型函数
*/
fn generic<T>(_s: SGen<T>) {}

fn main() {
    reg_fn(S(A)); // 具体类型
    gen_spec_t(SGen(A)); // 隐式的指定类型参数 A
    gen_spec_i32(SGen(6)); // 隐式的指定类型参数 i32

    // 为generic显示的指定类型参数char
    generic::<char>(SGen('a'));
    // 隐式的指定参数类型char
    generic(SGen('c'))
}
