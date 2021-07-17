use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let mut b = true;
    for s_i in s {
        if s_i == '1' {
            break;
        } else {
            b = !b;
        }
    }
    let ans = if b { "Takahashi" } else { "Aoki" };
    println!("{}", ans);
}
