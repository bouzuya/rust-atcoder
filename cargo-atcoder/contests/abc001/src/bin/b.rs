use proconio::input;

fn main() {
    input! {
        m: usize,
    };
    let ans = if m < 100 {
        "00".to_string()
    } else if m <= 5_000 {
        format!("{:02}", m / 100)
    } else if m <= 30_000 {
        format!("{:02}", m / 1_000 + 50)
    } else if m <= 70_000 {
        format!("{:02}", (m / 1_000 - 30) / 5 + 80)
    } else {
        "89".to_string()
    };
    println!("{}", ans);
}
