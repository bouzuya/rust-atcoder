use proconio::input;

fn prime_factorization(n: usize) -> Vec<(usize, usize)> {
    let mut p = vec![];
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c > 0 {
            p.push((i, c));
        }
    }
    if n != 1 {
        p.push((n, 1));
    }
    p
}

fn main() {
    input! {
        n: usize,
    };
    let mut dp = vec![1_usize; n + 1];
    for i in 2..=n {
        let p = prime_factorization(i);
        for &(p_i, c_i) in p.iter() {
            dp[p_i] += c_i;
        }
    }
    let ans = dp
        .iter()
        .fold(1_usize, |acc, &dp_i| (acc * dp_i) % 1_000_000_007);
    println!("{}", ans);
}
