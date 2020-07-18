use proconio::input;

fn main() {
    input! {
        n: i64,
    };
    let mut p = 1;
    for i in 1..=n {
        p *= i;
        p %= 1_000_000_007;
    }
    let ans = p;
    println!("{}", ans);
}
