use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        cp: [(Usize1, usize); n],
        q: usize,
        lr: [(Usize1, usize); q],
    };

    let mut sum = vec![vec![0; n + 1]; 2];
    for (i, (c, p)) in cp.iter().copied().enumerate() {
        sum[c][i + 1] = sum[c][i] + p;
        sum[1 - c][i + 1] = sum[1 - c][i];
    }

    for (l, r) in lr {
        let a = sum[0][r] - sum[0][l];
        let b = sum[1][r] - sum[1][l];
        println!("{} {}", a, b);
    }
}
