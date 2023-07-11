use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        a: Chars,
        b: Usize1,
    };
    let n = a.len();
    let ans = a[b % n];
    println!("{}", ans);
}
