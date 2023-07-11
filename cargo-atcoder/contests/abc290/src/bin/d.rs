use proconio::{input, marker::Usize1};

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        t: usize,
        ndk: [(usize, usize, Usize1); t],
    };
    for (n, d, k) in ndk {
        let d = d % n;
        let ans = if d == 0 {
            k
        } else {
            (d * k) % n + k / (n / gcd(n, d))
        };
        println!("{}", ans);
    }
}
