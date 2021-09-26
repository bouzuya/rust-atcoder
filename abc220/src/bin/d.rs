use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mod_p = 998_244_353;
    let mut dp = vec![0; 10];
    dp[a[0]] = 1;
    for i in 0..n - 1 {
        let mut next = vec![0; 10];
        for x in 0..10 {
            if dp[x] > 0 {
                let y = a[i + 1];
                next[(x + y) % 10] += dp[x];
                next[(x + y) % 10] %= mod_p;
                next[(x * y) % 10] += dp[x];
                next[(x * y) % 10] %= mod_p;
            }
        }
        dp = next;
    }

    for d in dp {
        println!("{}", d);
    }
}
