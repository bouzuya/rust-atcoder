use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = if a + b == 15 {
        "+"
    } else if a * b == 15 {
        "*"
    } else {
        "x"
    };
    println!("{}", ans);
}
