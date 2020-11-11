use proconio::input;

fn main() {
    input! {
        a: i64,
        b: i64,
    };
    let ans = if a == 0 || b == 0 || (a > 0 && b < 0) || (a < 0 && b > 0) {
        "Zero"
    } else if a > 0 && b > 0 {
        "Positive"
    } else if a < 0 && b < 0 {
        if (b - a - 1) % 2 == 0 {
            "Positive"
        } else {
            "Negative"
        }
    } else {
        unreachable!()
    };
    println!("{}", ans);
}
