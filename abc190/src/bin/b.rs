use proconio::input;

fn main() {
    input! {
        n: usize,
        s: i64,
        d: i64,
        xy: [(i64, i64); n],
    };
    let ans = xy.iter().any(|&(x_i, y_i)| x_i < s && y_i > d);
    println!("{}", if ans { "Yes" } else { "No" });
}
