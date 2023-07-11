use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        x: usize,
        y: usize,
    };
    let ans = (x / a).min(y / b);
    println!("{}", ans);
}
