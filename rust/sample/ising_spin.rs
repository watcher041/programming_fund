
// マクロの定義
macro_rules! macro_var {
    (cat) => { "cat" };
    (dog) => { "dog" };
}
const T_MAX:f64 = 3.6;

// メイン関数
fn main()
{
    println!("{}",macro_var!(cat));
}