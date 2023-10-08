use ac_library::ModInt998244353 as ModInt;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        t: [usize; n],
    };
    let zero = ModInt::new(0);
    let one = ModInt::new(1);
    let invn = ModInt::new(n).inv();
    let mut dp = vec![zero; x + 1];
    dp[0] = one;
    for i in 1..=x {
        for t_j in t.iter().copied() {
            if i >= t_j {
                let p = dp[i - t_j] * invn;
                dp[i] += p;
            }
        }
    }
    let mut ans = zero;
    for i in 0..=x {
        if i + t[0] > x {
            ans += dp[i] * invn;
        }
    }
    println!("{}", ans);
}
