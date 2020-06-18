use proconio::input;

fn main() {
    input! {
        r: i64,
        g: i64,
        b: i64,
    };
    let ans = (r * 100 + g * 10 + b) % 4 == 0;
    println!("{}", if ans { "YES" } else { "NO" });
}
