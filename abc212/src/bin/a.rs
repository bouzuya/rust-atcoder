use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if a > 0 && b == 0 {
        "Gold"
    } else if a == 0 && b > 0 {
        "Silver"
    } else {
        "Alloy"
    };
    println!("{}", ans);
}
