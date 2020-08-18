use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        a: usize,
        _: usize,
        s: Chars
    };
    let mut is_ok = true;
    for (i, &s_i) in s.iter().enumerate() {
        if i < a {
            if !s_i.is_digit(10) {
                is_ok = false;
                break;
            }
        } else if i == a {
            if s_i != '-' {
                is_ok = false;
                break;
            }
        } else {
            if !s_i.is_digit(10) {
                is_ok = false;
                break;
            }
        }
    }
    println!("{}", if is_ok { "Yes" } else { "No" });
}
