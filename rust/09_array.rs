
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

    // numberが1以上10以下の間でループを続ける（次のループではnumberが1足される）
    for number in 1..11 {
        println!("for：今{}回目です",number);
    }
}
