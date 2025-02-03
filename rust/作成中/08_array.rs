

// 配列、ベクタ、スライスを利用するプログラム
fn main()
{
    // 配列の初期化
    let x = [9,8,7,6];
    print_typename(x); // [i32;4]
    println!("x={:?}",x); // {} だと文字列として認識してエラーになるため、{:?}とすることで事前に型を識別して表示する

    let y = [1.2,3.4,4.5,6.7];
    print_typename(y); // [f64;4]
    println!("y={:?}",y); // {} だと文字列として認識してエラーになるため、{:?}とすることで事前に型を識別して表示する

    // 配列の宣言
    let a: [i32; 3] = [0, 1, 2];
    println!("a={:?}",a);

    // 二重配列の宣言
    let b: [[i32; 3]; 8] = [[0;3]; 8];
    println!("b={:?}",b);

    // 配列の一番最初の型と違う型のものがあればコンパイルエラーになる
    // let z = [1,3,4.'s',"s"."string"];
    // print_typename(z);

    // 配列の要素を取得する場合
    println!("1番目：{}、2番目：{}",x[0],x[1]);

    // 配列の要素をループで取得する場合
    println!("配列のインデックスでループして、インデックスから要素を取得");
    for index in 0..x.len() {
        println!("{}番目：{}",index+1,x[index]);
    }
    println!("配列の要素をループして、要素を取得");
    for item in x {
        println!("要素のみ1：{}",item);
    }
    for item in x.iter() {
        println!("要素のみ2：{}",item);
    }
    println!("配列のインデックスとそのときの要素でループして、インデックスと要素を取得");
    for (index, item) in x.iter().enumerate() {
        println!("{}番目：{}",index+1,item);
    }
    
    // 配列では途中で要素を追加、削除できないので、その場合はvec!を利用
    let mut z = vec![5,4,3,2,1];
    print_typename(z.clone()); // vec!の型を見るには、cloneをつけないとコンパイルエラーになる（alloc::vec::Vec<i32>）
    z.push(6);
    println!("z={:?}",z);
    z.remove(5);
    println!("z={:?}",z);

}

// このようにすることで、引数が渡るとTにその引数の型が入る（ジェネリック）
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}