use proconio::input;

fn gcd(n: usize, m: usize) -> usize {
    if m == 0 {
        n
    } else {
        gcd(m, n % m)
    }
}

fn lcm(n: usize, m: usize) -> usize {
    n / gcd(n, m) * m
}

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let f = |x: usize| -> usize { x - ((x / c) + (x / d) - (x / lcm(c, d))) };
    let ans = f(b) - f(a - 1);
    println!("{}", ans);
}
