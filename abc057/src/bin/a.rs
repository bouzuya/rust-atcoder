use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = (a + b) % 24;
    println!("{}", ans);
}
