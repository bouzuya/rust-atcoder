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
        a: usize,
        b: usize,
        c: usize,
    };
    let bc = pow(b, c, 4);
    let bc = if bc == 0 { 4 } else { bc };
    let a = a % 10;
    let ans = a.pow(bc as u32) % 10;
    println!("{}", ans);
}
