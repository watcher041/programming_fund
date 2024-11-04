

// 所有権を考慮したプログラム
fn main()
{
    let mut x = vec![5,4,3,2,1];

    // vec!型の変数を別の関数に渡すと、次に同じ変数を利用した場合にエラーになる
    // print_typename(x); 
    // エラー内容は以下の通り
    // x.push(6);
    //     ^ value borrowed here after move
    // 所有権が移った（main関数内のx→print_typenameの_）ことで、その変数は使えなくなる。
    // そのため、代わりにコピーを引数として渡すことで所有権が移らないようにする
    // あるいは、一度そのまま渡して返り値でもらう必要がある。
    print_typename(x.clone()); 
    x.push(6);
    println!("x={:?}",x);

    // 長さが固定されている変数（数値型、配列など）は暗黙的にコピーされる
    let mut y = [1,2,3];
    print_typename(y); 

    y[0] = 5;
    println!("y={:?}",y);

}

// このようにすることで、引数が渡るとTにその引数の型が入る（ジェネリック）
fn print_typename<T>(_: T) {
    println!("{}", std::any::type_name::<T>());
}