use proconio::input;

fn main() {
    input! {
        x: i64
    };
    // 5 -> 6 -> 5 -> 6 ...
    // 1 2 3 4 5 6 7 8 9 10 11 12
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
