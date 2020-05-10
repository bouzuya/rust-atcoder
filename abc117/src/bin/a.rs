use proconio::input;

fn main() {
    input! {
        t: f64,
        x: f64,
    };
    let ans = t / x;
    println!("{}", ans);
}
