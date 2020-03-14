use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = if a + b < c && 4 * a * b < (c - a - b).pow(2) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
