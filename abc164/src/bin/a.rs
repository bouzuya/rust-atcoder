use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    };
    let ans = if s > w { "safe" } else { "unsafe" };
    println!("{}", ans);
}
