
// 利用するクレイトを指定
use std::fs::File;
use std::io::Read;
use std::io::Write;
use rand::prelude::*;

// マクロの定義
macro_rules! param {
            (N_MAX) => {36};
            (SCALE) => {0.1};
        (COUNT_MAX) => {10};
               (Lx) => {4};
               (Ly) => {4};
          (MCS_MAX) => {13000};
                (J) => {1.0};
         (HEAT_MAX) => {3000};
         (MAG_FILE) => {"mag_file.dat"};
         (CAL_FILE) => {"cal_file.dat"};
          (MS_FILE) => {"ms_file.dat"};
    (SPIN_STEPFILE) => {"step_spin.dat"};
     (ENG_STEPFILE) => {"step_eng.dat"};
}

// メイン関数
fn main()
{
    let mut spin = [[0.0;param!(Lx)];param!(Ly)];
    let mut T:f64;
    let mut beta:f64; 
    // let mut E_ave:f64;
    // let mut M_ave:f64;
    // let mut m:f64;
    // let mut beta:f64;
    // let mut fmag = File::create(param!(MAG_FILE))
    //                     .expect("file not found");
    // let mut fcal = File::open(param!(CAL_FILE))
    //                     .expect("file not found");
    // let mut fms  = File::open(param!(MS_FILE))
    //                     .expect("file not found");

    // 各温度ごとのデータをとる
    for n in 1..param!(N_MAX) {

        // 温度とその逆数βを計算
        T = (n as f64) * param!(SCALE);
        beta = 1.0 / T;
        println!("\n 温度={}",T);

        for count in 1..param!(COUNT_MAX) {

            // スピンの配置を初期化する
            spin = spin_initialization();
            println!("spin={:?}",spin);
        }
    }
}

// 
fn spin_initialization() -> [[f64;param!(Lx)];param!(Ly)]
{
    let mut spin = [[0.0;param!(Lx)];param!(Ly)];

    // 乱数列を初期化
    let mut rng = rand::thread_rng(); // デフォルトの乱数生成器を初期化します
    let i: i32 = rng.gen();           // 整数値の乱数を生成する
    let f: f32 = rng.gen();           // 浮動小数点数の乱数を生成する

    return spin;
}

// fn spin_judge(){}
// fn mcs_physics(){}
// fn spin_arrangement(){}
// fn physics_quentities(){}
// fn date_graph(){}
// fn E(){}