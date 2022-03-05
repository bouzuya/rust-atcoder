use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        x: usize,
    };
    let ans = if x <= a {
        1_f64
    } else if x <= b {
        (c as f64) / ((b - a) as f64)
    } else {
        0_f64
    };
    println!("{}", ans);
}
