use proconio::input;

fn main() {
    input! {
        x_1: i64,
        y_1: i64,
        x_2: i64,
        y_2: i64,
    };
    let ans = (x_1 - x_2).abs() + (y_1 - y_2).abs() + 1;
    println!("{}", ans);
}
