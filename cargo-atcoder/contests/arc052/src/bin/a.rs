use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut digits = vec![];
    for c in s {
        if c.is_digit(10) {
            digits.push(c);
        }
    }
    let ans = digits.iter().collect::<String>().parse::<i64>().unwrap();
    println!("{}", ans);
}
