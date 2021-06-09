use proconio::input;

fn pow(b: usize, mut n: usize, m: usize) -> usize {
    let mut res = 1_usize;
    let mut x = b;
    while n > 0 {
        if n & 1 == 1 {
            res *= x;
            res %= m;
        }
        x *= x;
        x %= m;
        n >>= 1;
    }
    res
}

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let ans = (pow(10, n, m * m) / m) % m;
    println!("{}", ans);
}
