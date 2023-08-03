use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let p = 1_000_000_007;
    let mut all = 1_usize;
    let mut c9 = 1_usize;
    let mut c8 = 1_usize;
    for _ in 0..n {
        all *= 10;
        all %= p;
        c9 *= 9;
        c9 %= p;
        c8 *= 8;
        c8 %= p;
    }
    let ans = (((p + ((p + all - c9) % p) - c9) % p) + c8) % p;
    println!("{}", ans);
}
