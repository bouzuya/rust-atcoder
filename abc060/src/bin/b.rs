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

fn main() {
    input! {
        a: u64,
        b: u64,
        c: u64,
    };
    let ans = c % gcd(a, b) == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
