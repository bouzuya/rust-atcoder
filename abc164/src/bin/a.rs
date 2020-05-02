use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    };
    let ans = if s <= w { "unsafe" } else { "safe" };
    println!("{}", ans);
}
