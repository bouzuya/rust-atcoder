use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    };
    if s.len() == 1 {
        println!("{}", if s[0] == '0' { 0 } else { 1 });
        return;
    }
    let mut count = 0;
    let mut contains_zero = false;
    for i in (0..s.len() - 1).step_by(2) {
        match s[i + 1] {
            '+' => {
                if !contains_zero && s[i] != '0' {
                    count += 1;
                }
                contains_zero = false;
            }
            '*' => {
                if s[i] == '0' {
                    contains_zero = true;
                }
            }
            _ => unreachable!(),
        }
    }
    match s[s.len() - 2] {
        '+' => {
            if s[s.len() - 1] != '0' {
                count += 1;
            }
        }
        '*' => {
            if !contains_zero && s[s.len() - 1] != '0' {
                count += 1;
            }
        }
        _ => unreachable!(),
    }
    let ans = count;
    println!("{}", ans);
}
