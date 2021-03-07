use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        k: usize,
        a: usize,
        b: usize,
    };
    let ans = if k < a + 1 || b <= a + 2 {
        k + 1
    } else {
        assert!(k >= a + 1 && b > a + 2);
        let k2 = k - (a + 1);
        max(k + 1, b + (k2 / 2) * (b - a) + k2 % 2)
    };
    println!("{}", ans);
}
