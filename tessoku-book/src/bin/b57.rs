use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    let m = 30;
    let mut dp = vec![vec![0_usize; n + 1]; m + 1];
    for i in 1..=n {
        dp[0][i] = i - i
            .to_string()
            .chars()
            .map(|c| (c as u8 - b'0') as usize)
            .sum::<usize>();
    }
    for i in 1..=m {
        for j in 0..=n {
            dp[i][j] = dp[i - 1][dp[i - 1][j]];
        }
    }

    for i in 1..=n {
        let mut cur = i;
        let mut cnt = k;
        for dp_j in dp.iter() {
            if (cnt & 1) != 0 {
                cur = dp_j[cur];
            }
            cnt >>= 1;
        }
        println!("{}", cur);
    }
}
