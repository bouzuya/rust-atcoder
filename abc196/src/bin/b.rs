use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        x: Chars,
    };
    let mut s = vec![];
    for &c in x.iter() {
        if c == '.' {
            break;
        }
        s.push(c);
    }
    let ans = s.iter().collect::<String>();
    println!("{}", ans);
}
