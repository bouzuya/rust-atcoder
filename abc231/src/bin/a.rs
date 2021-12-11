use proconio::input;

fn main() {
    input! {
        d: usize,
    }
    let ans = d as f64 / 100_f64;
    println!("{}", ans);
}
