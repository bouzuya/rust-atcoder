use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
    };

    let mut edges = vec![vec![0_usize; n]; n];
    for i in 0..n - 1 {
        input! {
            d_i: [usize; n - 1 - i],
        }
        for j in 0..n - 1 - i {
            edges[i][i + 1 + j] = d_i[j];
            edges[i + 1 + j][i] = d_i[j];
        }
    }

    let mut dp = vec![0_usize; 1 << n];

    for bits in 0..1 << n {
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let bit_i = 1_usize << i;
                let bit_j = 1_usize << j;
                if (bits & bit_i) == 0 || (bits & bit_j) == 0 {
                    continue;
                }
                chmax!(dp[bits], dp[bits ^ bit_i ^ bit_j] + edges[i][j]);
            }
        }
    }

    let ans = dp.iter().copied().max().unwrap();
    println!("{}", ans);
}
