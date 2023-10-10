use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
    };
    let ans = x.max(y) - x.min(y) > 1;
    println!("{}", if ans { "Alice" } else { "Brown" });
}
