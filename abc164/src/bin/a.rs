use proconio::input;

fn main() {
    input! {
        s: usize,
        w: usize,
    };
    let ans = s <= w;
    println!("{}", if ans { "unsafe" } else { "safe" });
}
