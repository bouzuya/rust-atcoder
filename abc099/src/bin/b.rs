use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let d = b - a;
    let ans = (d - 1) * d / 2 - a;
    println!("{}", ans);
}
