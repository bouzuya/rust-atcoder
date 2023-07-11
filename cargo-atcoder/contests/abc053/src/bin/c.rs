use proconio::input;

fn main() {
    input! {
        x: i64
    };
    let a = x / (5 + 6);
    let b = x % (5 + 6);
    let ans = a * 2
        + if b == 0 {
            0
        } else if b <= 6 {
            1
        } else {
            2
        };
    println!("{}", ans);
}
