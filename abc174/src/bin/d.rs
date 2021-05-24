use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        c: Chars,
    };
    let count_r = c.iter().filter(|&&c_i| c_i == 'R').count();
    let ans = c.iter().take(count_r).filter(|&&c_i| c_i == 'W').count();
    println!("{}", ans);
}
