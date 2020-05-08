use proconio::input;

fn main() {
    input! {
        r: i64,
        g: i64,
    };
    let ans = 2 * g - r;
    println!("{}", ans);
}
