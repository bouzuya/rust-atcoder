use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = ((a - c).abs() <= d) || ((a - b).abs() <= d && (b - c).abs() <= d);
    println!("{}", if ans { "Yes" } else { "No" });
}
