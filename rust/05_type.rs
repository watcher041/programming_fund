
// 変数の型を表示するプログラム
fn main()
{
    let x = 1;
    print_typename(x); // i32

    let y = 3.2;
    print_typename(y); // f64

    let z = true;
    print_typename(z); // bool

    let s1 = 's';
    let s2 = "s";
    print_typename(s1); // char
    print_typename(s2); // &str

    let t = "Hello World!";
    print_typename(t); // &str
}

// このようにすることで、引数が渡るとTにその引数の型が入る（ジェネリック）
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}