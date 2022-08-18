use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut rev = HashMap::new();
    let mut map = HashMap::new();
    for (s_i, t_i) in s.iter().copied().zip(t.iter().copied()) {
        match map.get(&s_i) {
            Some(x) => {
                if x == &t_i {
                    continue;
                }
                println!("No");
                return;
            }
            None => {
                map.insert(s_i, t_i);
            }
        }
        match rev.get(&t_i) {
            Some(x) => {
                if x == &s_i {
                    continue;
                }
                println!("No");
                return;
            }
            None => {
                rev.insert(t_i, s_i);
            }
        }
    }

    println!("Yes");
}
