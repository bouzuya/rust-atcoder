use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars,
    };
    let ans = if (n[0] == n[1] && n[0] == n[2]) || (n[1] == n[2] && n[1] == n[3]) {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
