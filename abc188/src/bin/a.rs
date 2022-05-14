use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let max = x.max(y);
    let min = x.min(y);
    let ans = min + 3 > max;
    println!("{}", if ans { "Yes" } else { "No" });
}
