use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        z: usize,
    };
    let t = y * z / x;
    let ans = if t * x == y * z { t - 1 } else { t };
    println!("{}", ans);
}
