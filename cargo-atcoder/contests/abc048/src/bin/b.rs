use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
        x: i64,
    };
    let c_b = b / x;
    let c_a = a.saturating_sub(1) / x;
    let ans = c_b - if a == 0 { -1 } else { c_a };
    println!("{}", ans);
}
