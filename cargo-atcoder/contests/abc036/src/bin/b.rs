use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    for j in 0..n {
        for i in (0..n).rev() {
            print!("{}", s[i][j]);
        }
        println!();
    }
}
