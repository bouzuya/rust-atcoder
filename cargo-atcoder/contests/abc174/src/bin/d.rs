use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        c: Chars,
    };
    let count_r = c.iter().copied().filter(|c_i| c_i == &'R').count();
    let ans = c
        .iter()
        .take(count_r)
        .copied()
        .filter(|c_i| c_i == &'W')
        .count();
    println!("{}", ans);
}
