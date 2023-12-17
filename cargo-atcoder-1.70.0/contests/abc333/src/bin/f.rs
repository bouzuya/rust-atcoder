use ac_library::ModInt998244353 as ModInt;
use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let a = ModInt::new(2).inv();
    let mut pa = vec![ModInt::new(1); n + 1];
    for i in 0..n {
        pa[i + 1] = pa[i] * a;
    }

    let mut dp = vec![ModInt::new(1); 1];
    for k in 2..=n {
        let mut next = vec![ModInt::new(0); k];
        let x = (ModInt::new(2) - a.pow((k - 1) as u64)).inv();

        for i in 0..k - 1 {
            next[0] += pa[k - 1 - i] * dp[i];
        }
        for i in 0..k - 1 {
            let mut now = next[i];
            now -= dp[i] * pa[k - 1];
            now *= a;
            now += dp[i];
            next[i + 1] = now;
        }
        for i in next.iter_mut() {
            *i *= x;
        }

        dp = next;
    }
    for i in 0..n {
        println!("{}", dp[i]);
    }
}
