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
    let x = gcd(gcd(a, b), c);
    let ans = (a / x - 1) + (b / x - 1) + (c / x - 1);
    println!("{}", ans);
}
