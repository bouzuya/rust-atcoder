use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        st: [(usize, usize); n - 1],
    };
    for i in 0..n - 1 {
        let (s, t) = st[i];
        a[i + 1] += a[i] / s * t;
    }
    let ans = a[n - 1];
    println!("{}", ans);
}
