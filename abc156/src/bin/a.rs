use proconio::input;

fn main() {
    input! {
        n: i64,
        r: i64,
    };
    let ans = if n >= 10 { r } else { r + (100 * (10 - n)) };
    println!("{}", ans);
}
