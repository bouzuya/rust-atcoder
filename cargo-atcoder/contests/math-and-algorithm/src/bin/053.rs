use proconio::input;

fn main() {
    input! {
        n: usize,
    };

    let modp = 1_000_000_007_usize;
    let pow = |mut r: usize, mut q: usize| -> usize {
        let mut p = 1_usize;
        for _ in 0..60 {
            if (q & 1) != 0 {
                p *= r;
                p %= modp;
            }
            r *= r;
            r %= modp;
            q >>= 1;
        }
        p
    };

    // S      = 4^0 + 4^1 + 4^2 + ... + 4^N
    //
    // 4S     =       4^1 + 4^2 + ... + 4^N + 4^{N+1}
    //
    // 4S - S = 4^{N+1} - 4^0
    // 3S     = 4^{N+1} - 1
    //  S     = (4^{N+1} - 1) / 3
    //
    // (4^{n+1} - 1) / 3
    let x = pow(4, n + 1);
    let x = (x + modp - 1) % modp;
    let x = (x * pow(3, modp - 2)) % modp;
    let ans = x;
    println!("{}", ans);
}
