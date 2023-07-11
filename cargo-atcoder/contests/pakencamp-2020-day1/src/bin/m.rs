use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut dp = vec![0; 1 << 20];
    for a_i in a {
        dp[a_i] = a_i;
    }
    let mut ans = 0_usize;
    for bits in 1..1 << 20 {
        if dp[bits] == bits {
            ans += 1;
        }
        for i in 0..20 {
            dp[bits | (1 << i)] |= dp[bits];
        }
    }
    println!("{}", ans);
}
