use proconio::input;

fn main() {
    input! {
        n: i64,
        k: i64,
    };
    let ans = if k == 1 { 0 } else { n - k };
    println!("{}", ans);
}
