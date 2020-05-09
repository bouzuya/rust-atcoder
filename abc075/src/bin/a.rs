use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
    };
    let ans = if a == b {
        c
    } else if a == c {
        b
    } else {
        a
    };
    println!("{}", ans);
}
