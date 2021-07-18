use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    };
    let p = s.iter().position(|&s_i| s_i == '1').unwrap() % 2 == 0;
    let ans = if p { "Takahashi" } else { "Aoki" };
    println!("{}", ans);
}
