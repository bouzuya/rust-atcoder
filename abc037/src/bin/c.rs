use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [i64; n],
    };
    let mut s = a[0..k].iter().sum::<i64>();
    let mut ans = s;
    for i_l in 0..n - k {
        s -= a[i_l];
        s += a[i_l + k];
        ans += s;
    }
    println!("{}", ans);
}
