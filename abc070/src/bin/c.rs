use proconio::input;

fn gcd(n: u64, m: u64) -> u64 {
    if n < m {
        gcd(m, n)
    } else if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn lcm(n: u64, m: u64) -> u64 {
    n / gcd(n, m) * m
}

fn main() {
    input! {
        n: usize,
        t: [u64; n],
    };
    let mut l = t[0];
    for &t_i in t.iter().skip(1) {
        l = lcm(l, t_i);
    }
    let ans = l;
    println!("{}", ans);
}
