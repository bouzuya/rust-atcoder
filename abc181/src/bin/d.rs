use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let ans = if s.len() == 1 {
        s[0] == '8'
    } else if s.len() == 2 {
        format!("{}{}", s[0], s[1]).parse::<usize>().unwrap() % 8 == 0
            || format!("{}{}", s[1], s[0]).parse::<usize>().unwrap() % 8 == 0
    } else {
        let count = |chars: &[char]| -> HashMap<usize, usize> {
            let mut counts = HashMap::new();
            for c in chars.iter().copied() {
                *counts.entry((c as u8 - b'0') as usize).or_insert(0) += 1_usize;
            }
            counts
        };

        let counts = count(&s);

        let mut ok = false;
        for i in 100..=999 {
            if i % 8 == 0 {
                let count_i = count(&i.to_string().chars().collect::<Vec<char>>());
                if count_i
                    .into_iter()
                    .all(|(d, c)| c <= (*counts.get(&d).unwrap_or(&0)))
                {
                    ok = true;
                    break;
                }
            }
        }
        ok
    };
    println!("{}", if ans { "Yes" } else { "No" });
}
