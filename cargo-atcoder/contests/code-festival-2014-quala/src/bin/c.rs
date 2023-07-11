use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let a = a - 1;
    let count_a = a / 4 - a / 100 + a / 400;
    let count_b = b / 4 - b / 100 + b / 400;
    let ans = count_b - count_a;
    println!("{}", ans);
}
