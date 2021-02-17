use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    let mut x = 0;
    for s_i in s {
        if x == 0 && (s_i == 'i' || s_i == 'I') {
            x += 1;
        }
        if x == 1 && (s_i == 'c' || s_i == 'C') {
            x += 1;
        }
        if x == 2 && (s_i == 't' || s_i == 'T') {
            x += 1;
        }
    }
    let ans = x == 3;
    println!("{}", if ans { "YES" } else { "NO" });
}
