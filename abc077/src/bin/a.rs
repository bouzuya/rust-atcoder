use proconio::{input, marker::Chars};

fn main() {
    input! {
        c: [Chars; 2],
    };
    let mut ok = true;
    for i in 0..2 {
        for j in 0..3 {
            if c[i][j] != c[2 - 1 - i][3 - 1 - j] {
                ok = false;
            }
        }
    }
    let ans = ok;
    println!("{}", if ans { "YES" } else { "NO" });
}
