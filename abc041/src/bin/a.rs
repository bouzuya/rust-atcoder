use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        s: Chars,
        i: Usize1,
    };
    let ans = s[i];
    println!("{}", ans);
}
