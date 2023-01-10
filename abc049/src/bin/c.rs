use proconio::{input, marker::Chars};

fn f(s: &mut Vec<char>) -> bool {
    let w = vec!["dream", "dreamer", "erase", "eraser"]
        .iter()
        .map(|w_i| w_i.chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();
    while !s.is_empty() {
        match s.last().unwrap() {
            'm' => {
                if s.ends_with(&w[0]) {
                    for _ in 0..w[0].len() {
                        s.pop();
                    }
                } else {
                    return false;
                }
            }
            'e' => {
                if s.ends_with(&w[2]) {
                    for _ in 0..w[2].len() {
                        s.pop();
                    }
                } else {
                    return false;
                }
            }
            'r' => {
                if s.ends_with(&w[1]) {
                    for _ in 0..w[1].len() {
                        s.pop();
                    }
                } else if s.ends_with(&w[3]) {
                    for _ in 0..w[3].len() {
                        s.pop();
                    }
                } else {
                    return false;
                }
            }
            _ => {
                return false;
            }
        }
    }
    true
}

fn main() {
    input! {
        mut s: Chars,
    };
    let ans = f(&mut s);
    println!("{}", if ans { "YES" } else { "NO" });
}
