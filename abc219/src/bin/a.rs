use proconio::input;

fn main() {
    input! {
        x: usize,
    };
    let ans = if x < 40 {
        format!("{}", 40 - x)
    } else if x < 70 {
        format!("{}", 70 - x)
    } else if x < 90 {
        format!("{}", 90 - x)
    } else {
        format!("expert")
    };
    println!("{}", ans);
}
