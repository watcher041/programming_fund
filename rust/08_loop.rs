
// 10回表示するプログラム
fn main()
{
    let number = 1;
    loop{
        println!("今{}回目です",number);
        number += 1; // number = number + 1
        if number == 10 {
            break;
        }
    }
}
