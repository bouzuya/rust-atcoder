use proconio::input;

fn main() {
    input! {
        n: usize,
        p: [usize; n],
    }

    let mut dp = vec![false; 100 * 100 + 1];
    dp[0] = true;
    for p_i in p {
        let mut next = vec![false; 100 * 100 + 1];
        for j in 0..dp.len() {
            if !dp[j] {
                continue;
            }
            next[j + p_i] = true;
            next[j] = true;
        }
        dp = next;
    }
    let ans = dp.into_iter().filter(|b| *b).count();
    println!("{}", ans);
}
