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
    let lcm = a * b / gcd(a, b);
    let ans = lcm;
    println!("{}", ans);
}
