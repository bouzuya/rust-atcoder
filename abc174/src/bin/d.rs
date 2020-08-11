use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        c: Chars,
    };
    let count_r = c.iter().filter(|&&c_i| c_i == 'R').count();
    let ans = c[0..count_r].iter().filter(|&&c_i| c_i == 'W').count();
    println!("{}", ans);
}
