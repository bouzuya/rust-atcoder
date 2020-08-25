use proconio::input;

fn main() {
    input! {
        s: i64,
    };
    let ans = 50_f64 / s as f64;
    println!("{}", ans);
}
