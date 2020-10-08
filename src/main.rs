use std::io;

fn main() {

    // Inputの処理
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("stdinの読み込みエラー");
    // usize型に変換
    let x = input_text.trim().parse::<usize>().expect("数値以外の入力");

    // 2からxまでの全ての整数のvec
    let mut l = (2..=x).collect::<Vec<usize>>();

    // Outputの初期化
    let mut s: Vec<usize> = Vec::with_capacity(x);

    let mut y;
    while l.len() != 0 {
        // yにLの始めの値の代入
        y = l[0];

        // lにおけるn割るyの余りが0でない値の保持
        l.retain(|&n| n % y != 0);

        // Sにyを追加
        s.push(y);

        // 途中経過
        println!("L = {:?}", l);
    }

    // 素数の表示
    println!("\nS = {:?}", s);
}
