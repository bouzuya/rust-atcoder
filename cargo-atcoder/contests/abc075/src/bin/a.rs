use proconio::input;

fn main() {
    input! {
        (a, b, c): (i64, i64, i64)
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
