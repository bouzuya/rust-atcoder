use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut p = 0;
    let mut c = 'g';
    for i in s {
        match (i, c) {
            ('g', 'g') => p += 0,
            ('g', 'p') => p += 1,
            ('p', 'g') => p -= 1,
            ('p', 'p') => p += 0,
            _ => unreachable!(),
        }
        c = if c == 'g' { 'p' } else { 'g' };
    }
    let ans = p;
    println!("{}", ans);
}
