use proconio::input;

fn main() {
    input! {
        a: [[i64; 4]; 4],
    };
    let mut gameover = true;
    for i in 0..4 {
        for j in 0..4 - 1 {
            if a[i][j] == a[i][j + 1] {
                gameover = false;
            }
            if a[j][i] == a[j + 1][i] {
                gameover = false;
            }
        }
    }
    let ans = gameover;
    println!("{}", if ans { "GAMEOVER" } else { "CONTINUE" });
}
