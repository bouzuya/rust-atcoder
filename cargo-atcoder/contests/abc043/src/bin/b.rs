use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut buf = vec![];
    for c in s {
        match c {
            '0' => {
                buf.push('0');
            }
            '1' => {
                buf.push('1');
            }
            'B' => {
                if buf.is_empty() {
                    // do nothing
                } else {
                    buf.pop();
                }
            }
            _ => {
                unreachable!();
            }
        }
    }

    let ans = buf.into_iter().collect::<String>();
    println!("{}", ans);
}
