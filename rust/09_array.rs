

// 配列、ベクタ、タプルを利用するプログラム
fn main()
{
    let x =[9,8,7,6];
    print_typename(x); // [i32;4]

    let y = [1.2,3.4,4.5,6.7];
    print_typename(y); // [f64;4]

    // 配列の一番最初の型と違う型のものがあればコンパイルエラーになる
    // let z = [1,3,4.'s',"s"."string"];
    // print_typename(z);

    // 配列の値を取得する場合
    println!("1番目：{}、2番目：{}",x[0],x[1]);

    // 配列の値をループで取得する場合
    println!("配列のインデックスでループして、インデックスから値を取得");
    for index in 0..x.len() {
        println!("{}番目：{}",index+1,x[index]);
    }
    println!("配列のインデックスとそのときの値でループして、値だけを取得");
    for (index, item) in x.iter().enumerate() {
        println!("{}番目：{}",index+1,item);
    }
    println!("配列の値をループして、値を取得");
    for item in &x {
        println!("{}",item);
    }
}

// このようにすることで、引数が渡るとTにその引数の型が入る（ジェネリック）
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}