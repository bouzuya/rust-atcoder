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
        a: usize,
        b: usize,
    };
    match (a / gcd(a, b)).checked_mul(b) {
        Some(x) if x <= 10_usize.pow(18) => println!("{}", x),
        Some(_) | None => println!("Large"),
    }
}
