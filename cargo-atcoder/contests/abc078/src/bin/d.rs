// 解説 AC <https://img.atcoder.jp/arc085/editorial.pdf>
use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        _z: i64,
        w: i64,
        a: [i64; n],
    };
    assert!(n > 0);
    let ans = if n == 1 {
        let z = a[0];
        let w = w;
        (z - w).abs()
    } else {
        let x1 = {
            let z = a[n - 2];
            let w = a[n - 1];
            (z - w).abs()
        };
        let x2 = {
            let z = a[n - 1];
            let w = w;
            (z - w).abs()
        };
        cmp::max(x1, x2)
    };
    println!("{}", ans);
}
