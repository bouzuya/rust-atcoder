use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let p = 1_000_000_007_usize;
    let mut all = 1_usize;
    let mut x = 1_usize;
    let mut y = 1_usize;
    for _ in 0..n {
        all *= 10_usize;
        all %= p;
        x *= 9_usize;
        x %= p;
        y *= 8_usize;
        y %= p;
    }
    let ans = (((((all + p - x) % p) + p - x) % p) + y) % p;
    println!("{}", ans);
}
