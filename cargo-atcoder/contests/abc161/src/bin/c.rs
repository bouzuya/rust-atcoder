use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    };
    let m = n % k;
    let ans = m.min(m + k).min((m - k).abs());
    println!("{}", ans);
}
