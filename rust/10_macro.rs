
// マクロを利用したプログラム
macro_rules! macro_var {
    (cat) => { "cat" };
    (dog) => { "dog" };
}

fn main()
{
    println!("{}",macro_var!(cat));
    println!("{}",macro_var!(dog));
}
