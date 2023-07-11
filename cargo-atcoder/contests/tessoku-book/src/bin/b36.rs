use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        k: usize,
        s: Chars,
    };
    let count_on = s.into_iter().filter(|&c| c == '1').count();
    let ans = count_on % 2 == k % 2;
    println!("{}", if ans { "Yes" } else { "No" });
}
