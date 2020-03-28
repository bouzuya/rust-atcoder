use proconio::input;
use proconio::marker::Bytes;

fn main() {
    input! {
        s: Bytes,
    };
    let ans = if s[2] == s[3] && s[4] == s[5] {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
