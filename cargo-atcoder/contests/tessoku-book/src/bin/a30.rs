use proconio::input;

// n^m (mod p)
fn mod_pow(mut b: usize, mut q: usize, p: usize) -> usize {
    let mut res = 1_usize;
    while q != 0 {
        if (q & 1) == 1 {
            res *= b;
            res %= p;
        }
        b = b.pow(2) % p;
        q >>= 1;
    }
    res
}

fn main() {
    input! {
        n: usize,
        r: usize,
    };

    let p = 1_000_000_007_usize;
    let mut fact = vec![1_usize; n + 1];
    for i in 1..=n {
        fact[i] = fact[i - 1] * i;
        fact[i] %= p;
    }
    let mut finv = vec![1_usize; n + 1];
    finv[n] = mod_pow(fact[n], p - 2, p);
    for i in (1..=n).rev() {
        finv[i - 1] = finv[i] * i;
        finv[i - 1] %= p;
    }

    let ans = (((fact[n] * finv[r]) % p) * finv[n - r]) % p;
    println!("{}", ans);
}
