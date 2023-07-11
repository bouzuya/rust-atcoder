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
        n: usize,
        a: [u64; n],
    };
    let mut g = a[0];
    for &a_i in a.iter().skip(1) {
        g = gcd(g, a_i);
    }
    println!("{}", g);
}
