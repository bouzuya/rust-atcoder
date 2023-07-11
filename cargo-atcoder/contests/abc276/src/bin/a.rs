use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let ans = s
        .iter()
        .rev()
        .position(|&c| c == 'a')
        .map(|i| (n - 1 - i) as i64 + 1)
        .unwrap_or(-1);
    println!("{}", ans);
}
