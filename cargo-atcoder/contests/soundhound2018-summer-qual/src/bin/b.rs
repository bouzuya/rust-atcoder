use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
        w: usize,
    };
    let mut ans = vec![];
    let mut i = 0;
    while i < s.len() {
        ans.push(s[i]);
        i += w;
    }
    println!("{}", ans.iter().collect::<String>());
}
