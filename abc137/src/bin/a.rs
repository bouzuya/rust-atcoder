use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = (a + b).max(a - b).max(a * b);
    println!("{}", ans);
}
