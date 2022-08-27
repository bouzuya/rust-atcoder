use proconio::input;

fn main() {
    input! {
        n: usize,
        txa: [(usize, usize, i64); n],
    };
    let none = -1_i64;
    let max_t = 100_000;
    let mut dp = vec![none; 5];
    let mut b = vec![vec![0; 5]; max_t + 1 + 1];
    for (t, x, a) in txa {
        b[t][x] = a;
    }
    dp[0] = 0_i64;
    for t in 0..=max_t {
        let mut next = vec![none; 5];
        for i in 0..5 {
            if dp[i] != none {
                for j in -1_i64..=1 {
                    if !(0..5).contains(&(i as i64 + j)) {
                        continue;
                    }
                    let ni = (i as i64 + j) as usize;
                    let nt = t + 1;
                    next[ni] = next[ni].max(dp[i] + b[nt][ni]);
                }
            }
        }
        dp = next;
    }

    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
