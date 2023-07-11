use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = if a + b >= 15 && b >= 8 {
        1 // アイスクリーム
    } else if a + b >= 10 && b >= 3 {
        2 // アイスミルク
    } else if a + b >= 3 {
        3 // ラクトアイス
    } else {
        4 // 氷菓
    };
    println!("{}", ans);
}
