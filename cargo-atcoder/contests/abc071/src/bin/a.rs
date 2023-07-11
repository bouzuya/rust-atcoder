use proconio::input;

fn main() {
    input! {
        x: i64,
        a: i64,
        b: i64,
    };
    let ans = if (x - a).abs() < (x - b).abs() {
        "A"
    } else {
        "B"
    };
    println!("{}", ans);
}
