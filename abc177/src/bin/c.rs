use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mod_p = 1_000_000_007_i64;
    let mut c = vec![0_i64; n + 1];
    for (i, &a_i) in a.iter().enumerate().rev() {
        c[i] = c[i + 1] + a_i;
        c[i] %= mod_p;
    }
    let mut s = 0_i64;
    for (i, &a_i) in a.iter().enumerate() {
        if i + 1 > n {
            continue;
        }
        s += (a_i * c[i + 1]) % mod_p;
        s %= mod_p;
    }
    let ans = s;
    println!("{}", ans);
}
