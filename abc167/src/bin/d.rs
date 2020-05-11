// 別解 https://atcoder.jp/contests/abc167/submissions/13117364

use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; n],
    };

    let kn = k.next_power_of_two().trailing_zeros() as usize;
    let mut tbl = vec![vec![n; n]; kn + 1];
    for j in 0..n {
        tbl[0][j] = a[j];
    }
    for i in 1..=kn {
        for j in 0..n {
            tbl[i][j] = tbl[i - 1][tbl[i - 1][j]];
        }
    }

    let mut c = 0;
    for i in 0..=kn {
        if (k >> i) & 1 == 1 {
            c = tbl[i][c];
        }
    }
    let ans = c + 1;
    println!("{}", ans);
}
