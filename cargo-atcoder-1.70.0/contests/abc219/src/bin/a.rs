use proconio::input;

fn main() {
    input! {
        x: usize,
    }
    let ans = if (0..40).contains(&x) {
        format!("{}", 40 - x)
    } else if (40..70).contains(&x) {
        format!("{}", 70 - x)
    } else if (70..90).contains(&x) {
        format!("{}", 90 - x)
    } else {
        "expert".to_string()
    };
    println!("{}", ans);
}
