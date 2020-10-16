use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
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
        a: usize,
        b: usize,
    };
    let ans = a * b / gcd(a, b);
    println!("{}", ans);
}
