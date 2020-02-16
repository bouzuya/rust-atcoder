use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    };
    let ans = if (a == b && b != c) || (a == c && a != b) || (b == c && a != b) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
