use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [i64; n],
    };
    let mut s = 0;
    let mut m = 1_000_i64;
    for i in 0..n {
        m += a[i] * s;
        s = 0;
        if i + 1 < n {
            if a[i] < a[i + 1] {
                s = m / a[i];
                m -= s * a[i];
            }
        }
    }
    let ans = m;
    println!("{}", ans);
}
