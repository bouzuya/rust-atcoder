use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        n: Chars
    };
    let ans = if n.iter().filter(|&&c| c == '7').count() > 0 {
        "Yes"
    } else {
        "No"
    };
    println!("{}", ans);
}
