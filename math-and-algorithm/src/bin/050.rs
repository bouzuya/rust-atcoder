use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let modp = 1_000_000_007_usize;
    // let mut p = 1_usize;
    // for _ in 0..b {
    //     p *= a;
    //     p %= modp;
    // }
    let mut p = 1;
    let mut m = a;
    let mut n = b;
    while n != 0 {
        if (n & 1) != 0 {
            p *= m;
            p %= modp;
        }
        m *= m;
        m %= modp;
        n >>= 1;
    }
    let ans = p;
    println!("{}", ans);
}
