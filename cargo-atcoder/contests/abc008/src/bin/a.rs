use proconio::input;

fn main() {
    input! {
        s: i64,
        t: i64,
    };
    let ans = t - (s - 1);
    println!("{}", ans);
}
