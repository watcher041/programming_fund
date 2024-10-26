
// 10回文字を表示するプログラム
fn main()
{
    let mut number = 1;

    // numberが10を超えればループを終了する
    loop{
        println!("loop：今{}回目です",number);
        number += 1; // number = number + 1
        if number > 10 {
            break;
        }
    }

    // numberが10以下ならループを続ける
    number = 1;
    while number <= 10 {
        println!("while：今{}回目です",number);
        number += 1;
    }
}
