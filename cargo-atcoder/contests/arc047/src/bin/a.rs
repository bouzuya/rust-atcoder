use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        l: usize,
        s: Chars,
    };
    let mut crash_count = 0;
    let mut count = 1;
    for c in s {
        match c {
            '+' => {
                count += 1;
            }
            '-' => {
                count -= 1;
            }
            _ => unreachable!("invalid char"),
        }
        if count > l {
            crash_count += 1;
            count = 1;
        }
    }
    let ans = crash_count;
    println!("{}", ans);
}
