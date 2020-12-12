use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut n = 0;
    for c in s {
        if !c.is_ascii_digit() {
            println!("error");
            return;
        }
        n *= 10;
        n += (c as u8 - '0' as u8) as usize;
    }
    let ans = n * 2;
    println!("{}", ans);
}
