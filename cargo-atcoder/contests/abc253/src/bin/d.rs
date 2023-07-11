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
        n: usize,
        a: usize,
        b: usize,
    };

    let f = |x: usize| x * (x + 1) / 2;

    let ab = a * b / gcd(a, b);
    let count_a = n / a;
    let count_b = n / b;
    let count_ab = n / ab;
    let sum_a = f(count_a) * a;
    let sum_b = f(count_b) * b;
    let sum_ab = f(count_ab) * ab;
    let sum_all = f(n);
    let ans = sum_all - (sum_a + sum_b - sum_ab);
    println!("{}", ans);
}
