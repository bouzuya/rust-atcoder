use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    };
    let ans = if a >= 13 {
        b
    } else if (6..=12).contains(&a) {
        b / 2
    } else {
        0
    };
    println!("{}", ans);
}
