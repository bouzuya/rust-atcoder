use proconio::{input, marker::Chars};

fn main() {
    input! {
        k: Chars,
        d: usize,
    }

    let modp = 1_000_000_007_usize;
    let mut dp = vec![vec![0_usize; 2]; d + 1];
    dp[0][0] = 1;
    for k_i in k.iter().copied() {
        let mut next = vec![vec![0_usize; 2]; d + 1];
        let k_i = (k_i as u8 - b'0') as usize;
        for j in 0..=d {
            for l in 0..=9 {
                let nj = (j + l) % d;
                next[nj][1] += dp[j][1];
                next[nj][1] %= modp;
                match l.cmp(&k_i) {
                    std::cmp::Ordering::Less => {
                        next[nj][1] += dp[j][0];
                        next[nj][1] %= modp;
                    }
                    std::cmp::Ordering::Equal => {
                        next[nj][0] += dp[j][0];
                        next[nj][0] %= modp;
                    }
                    std::cmp::Ordering::Greater => {
                        // do nothing
                    }
                }
            }
        }
        dp = next;
    }

    let ans = (dp[0][0] + dp[0][1] + modp - 1) % modp;
    println!("{}", ans);
}
