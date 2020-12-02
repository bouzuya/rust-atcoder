use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut prev = s[0];
    let mut count = 0;
    for &s_i in s.iter().skip(1) {
        if prev == s_i {
            continue;
        }
        count += 1;
        prev = s_i;
    }
    let ans = count;
    println!("{}", ans);
}
