use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
    }

    let chars = "JOI".chars().collect::<Vec<char>>();
    let mut dp = vec![0_usize; 1 << 3];
    dp[1] = 1_usize;
    for s_i in s {
        let mut next = vec![0_usize; 1 << 3];
        let mask = 1 << chars.iter().position(|c| c == &s_i).unwrap();
        for prev_bits in 0..1 << 3 {
            for next_bits in 0..1 << 3 {
                if (next_bits & mask) == 0 {
                    continue;
                }
                if (prev_bits & next_bits) == 0 {
                    continue;
                }
                next[next_bits] += dp[prev_bits];
                next[next_bits] %= 10_007;
            }
        }
        dp = next;
    }
    let ans = dp.iter().copied().sum::<usize>() % 10_007;
    println!("{}", ans);
}
