use proconio::input;

fn gcd(n: u64, m: u64) -> u64 {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn main() {
    input! {
        a: u64,
        b: u64,
    };
    // lcm(a, b) = a * b / gcd(a, b)
    let x = a / gcd(a, b);
    match x.checked_mul(b) {
        Some(lcm) if lcm <= 1_000_000_000_000_000_000 => println!("{}", lcm),
        Some(_) | None => println!("Large"),
    }
}
