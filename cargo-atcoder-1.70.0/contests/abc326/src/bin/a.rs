use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
    };
    let ans = if x < y {
        y - x <= 2
    } else if x > y {
        x - y <= 3
    } else {
        unreachable!()
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
