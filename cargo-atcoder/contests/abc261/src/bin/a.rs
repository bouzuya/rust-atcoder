use proconio::input;

fn main() {
    input! {
        l1: i64,
        r1: i64,
        l2: i64,
        r2: i64,
    };
    let ans = (r1.min(r2) - l1.max(l2)).max(0);
    println!("{}", ans);
}
