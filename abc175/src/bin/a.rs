use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let c = s.iter().filter(|&&c| c == 'R').count();
    let ans = if c == 2 {
        if s[0] == s[1] || s[1] == s[2] {
            c
        } else {
            1
        }
    } else {
        c
    };
    println!("{}", ans);
}
