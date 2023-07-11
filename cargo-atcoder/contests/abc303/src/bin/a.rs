use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    };
    let ans = s.into_iter().zip(t.into_iter()).all(|(s, t)| {
        s == t
            || (s == '1' && t == 'l')
            || (s == 'l' && t == '1')
            || (s == '0' && t == 'o')
            || (s == 'o' && t == '0')
    });
    println!("{}", if ans { "Yes" } else { "No" });
}
