use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    };
    let x_a = if a == 0 { -1 } else { (a - 1) / x };
    let x_b = b / x;
    let ans = x_b - x_a;
    println!("{}", ans);
}
