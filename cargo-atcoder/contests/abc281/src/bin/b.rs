use proconio::{input, marker::Chars};

fn f(s: &[char]) -> bool {
    if s.len() != 8 {
        return false;
    }

    if !s[0].is_ascii_uppercase() {
        return false;
    }

    if !('1'..='9').contains(&s[1]) {
        return false;
    }

    if s[1..7].iter().collect::<String>().parse::<usize>().is_err() {
        return false;
    }

    if !s[7].is_ascii_uppercase() {
        return false;
    }

    true
}

fn main() {
    input! {
        s: Chars,
    };
    let ans = f(&s);
    println!("{}", if ans { "Yes" } else { "No" });
}
