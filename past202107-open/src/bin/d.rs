use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s: Chars,
    };
    for i in 1..n - 1 {
        if s[i] == 'x' && s[i - 1] == s[i + 1] {
            let c = s[i - 1];
            if c == 'a' || c == 'i' || c == 'u' || c == 'e' || c == 'o' {
                s[i - 1] = '.';
                s[i] = '.';
                s[i + 1] = '.';
            }
        }
    }
    let ans = s.into_iter().collect::<String>();
    println!("{}", ans);
}
