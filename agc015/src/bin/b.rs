use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let n = s.len();
    let mut sum = 0;
    for (i, &c) in s.iter().enumerate() {
        let b = i;
        let t = n - i - 1;
        sum += match c {
            'U' => t + b * 2,
            'D' => t * 2 + b,
            _ => unreachable!(),
        };
    }
    let ans = sum;
    println!("{}", ans);
}
