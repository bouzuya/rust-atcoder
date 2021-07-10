use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    };
    let max = 5_000;
    let mut s = vec![vec![0; max + 1 + 1]; max + 1 + 1];
    for (a_i, b_i) in ab.iter().copied() {
        s[a_i][b_i] += 1;
    }

    for i in 0..max + 1 + 1 {
        for j in 0..max + 1 {
            s[i][j + 1] += s[i][j];
        }
    }
    for j in 0..max + 1 + 1 {
        for i in 0..max + 1 {
            s[i + 1][j] += s[i][j];
        }
    }

    let mut ans = 0;
    for i in 1..max + 1 + 1 {
        for j in 1..max + 1 + 1 {
            let (t, b) = (i - 1, cmp::min(i + k, max + 1));
            let (l, r) = (j - 1, cmp::min(j + k, max + 1));
            let a = s[t][l] + s[b][r] - s[t][r] - s[b][l];
            ans = cmp::max(ans, a);
        }
    }
    println!("{}", ans);
}
