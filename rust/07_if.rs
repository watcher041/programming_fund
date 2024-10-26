
// 変数の値に基づいて処理を分岐するプログラム
fn main()
{
    let number = 1;
    if number < 1 {
        println!("この数字は1未満の値です"); // number < 1
    }else if 1 <= number && number < 5 {
        println!("この数字は1以上、5未満の値です"); // 1 <= number AND number < 5
    }else if number != 5 {
        println!("この数字は5ではありません"); // number != 5
    }else if number < 1 || 5 <= number {
        println!("この数字は1未満あるいは5以上の値です"); // number < 1 OR 5 <= number
    }else{
        println!("この数字は範囲外の値です");
    }
}
