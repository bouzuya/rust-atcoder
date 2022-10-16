use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };

    let p = 1_000_000_007;
    let mut res = 1_usize;
    let mut q = b;
    let mut base = a;
    while q != 0 {
        if (q & 1) == 1 {
            res *= base;
            res %= p;
        }
        base = base.pow(2) % p;
        q >>= 1;
    }
    let ans = res;
    println!("{}", ans);
}
