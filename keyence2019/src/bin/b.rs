use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let t = "keyence".chars().collect::<Vec<char>>();
    let mut ans = false;
    for i in 0..7 {
        if s[0..i + 1] != t[0..i + 1] {
            continue;
        }
        let o = n - (7 - i);
        if s[o + 1..] != t[i + 1..] {
            continue;
        }
        ans |= true;
    }
    ans |= s[n - 7..] == t[0..];
    println!("{}", if ans { "YES" } else { "NO" });
}
