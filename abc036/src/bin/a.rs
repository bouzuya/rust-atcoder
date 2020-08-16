use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = (b + (a - 1)) / a;
    println!("{}", ans);
}
