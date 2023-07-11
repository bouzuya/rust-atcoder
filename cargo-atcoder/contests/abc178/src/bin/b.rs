use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        c: i64,
        d: i64,
    };
    let ans = (a * c).max(a * d).max(b * c).max(b * d);
    println!("{}", ans);
}
