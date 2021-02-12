use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        f: [[u8; 10]; n],
        p: [[i64; 11]; n],
    };
    let mut max_v = -1_000_000_000;
    for bits in 1..1 << 10 {
        let is = (0..10).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        let mut v = 0_i64;
        for i in 0..n {
            let mut count = 0;
            for j in 0..10 {
                if is[j] && f[i][j] == 1 {
                    count += 1;
                }
            }
            v += p[i][count];
        }
        max_v = max(max_v, v);
    }
    let ans = max_v;
    println!("{}", ans);
}
