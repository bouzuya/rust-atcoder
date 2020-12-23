use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s: Chars,
    };
    let mut count = 1;
    let mut p = s[0];
    for &c in s.iter().skip(1) {
        if p != c {
            p = c;
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
