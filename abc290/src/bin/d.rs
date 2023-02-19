use proconio::input;

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
        ndk: [(usize, usize, usize); t],
    };
    for (n, d, k) in ndk {
        let g = gcd(n, d);
        let d = d % n;
        let ans = if k == 1 {
            0
        } else if d == 0 {
            k - 1
        } else if g == 1 {
            (d * (k - 1)) % n
        } else {
            let c = n * d / g / d;
            ((d * (k - 1)) % n + (k - 1) / c) % n
        };
        println!("{}", ans);
    }
}
