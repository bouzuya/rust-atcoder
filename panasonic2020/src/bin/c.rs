use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = if a + b >= c {
        false
    } else {
        4 * a * b < (c - a - b) * (c - a - b)
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
