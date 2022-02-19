use proconio::input;

fn main() {
    input! {
        h: f64,
    };
    let ans = ((h + 12800000_f64) * h).sqrt();
    println!("{}", ans);
}
