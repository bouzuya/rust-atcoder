use proconio::input;

fn main() {
    input! {
        x: i64,
        y: i64,
        k: i64,
    };
    let ans = if y >= k { x + k } else { x + y - (k - y) };
    println!("{}", ans);
}
