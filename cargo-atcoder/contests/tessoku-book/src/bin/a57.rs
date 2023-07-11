use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [Usize1; n],
        xy: [(Usize1, usize); q],
    };

    let m = 30;
    let mut dp = vec![vec![0_usize; n]; m + 1];
    for (i, a_i) in a.iter().copied().enumerate() {
        dp[0][i] = a_i;
    }
    for i in 1..=m {
        for j in 0..n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for (x, y) in xy {
        let mut cur = x;
        let mut cnt = y;
        for dp_i in dp.iter() {
            if (cnt & 1) != 0 {
                cur = dp_i[cur];
            }
            cnt >>= 1;
        }
        println!("{}", cur + 1);
    }
}
