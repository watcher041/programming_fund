
// ファイルを利用するためのクレート（パッケージ）を指定
use std::fs::File;
use std::io::Read;
use std::io::Write;

// メイン関数
fn main()
{
    // ファイル名を指定
    let filename = "12_file.txt";

    // ファイル名を表示
    println!("In file {}", filename);

    // ファイルの有無を確認してから開く
    let mut f = File::open(filename)
                    .expect("file not found");        

    // ファイルの内容を確認してから読み込む
    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    // ファイルの内容を表示
    println!("With text:\n{}", contents);

    // ファイルの有無を確認してから開く
    let mut fc = File::create(filename)
                    .expect("file not found");       

    // ファイルに書き込む
    writeln!(fc, "hello rust world.")
           .expect("cannot write.");

    // ファイルの内容を表示
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");
    println!("With text:\n{}", contents);
}