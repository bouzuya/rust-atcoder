use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        _: usize,
        a: Usize1,
        b: Usize1,
        c: Usize1,
        d: Usize1,
        s: Chars,
    };
    if c < d {
        let mut is_ok = true;
        for i in a + 1..=c {
            if s[i - 1] == '#' && s[i] == '#' {
                is_ok = false;
                break;
            }
        }
        for i in b + 1..=d {
            if s[i - 1] == '#' && s[i] == '#' {
                is_ok = false;
                break;
            }
        }
        println!("{}", if is_ok { "Yes" } else { "No" });
    } else {
        let mut is_ok = true;
        for i in a + 1..=c {
            if s[i - 1] == '#' && s[i] == '#' {
                is_ok = false;
                break;
            }
        }
        let mut is_ng = true;
        for i in b + 1..=d + 1 {
            if s[i - 2] == '.' && s[i - 1] == '.' && s[i] == '.' {
                is_ng = false;
                break;
            }
        }
        println!("{}", if is_ok && !is_ng { "Yes" } else { "No" });
    }
}
