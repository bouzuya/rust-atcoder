use proconio::input;

fn main() {
    input! {
        d: usize,
        t: usize,
        s: usize,
    };
    let ans = (d + s - 1) / s <= t;
    println!("{}", if ans { "Yes" } else { "No" });
}
