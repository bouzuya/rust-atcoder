use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    };
    let mut map = HashMap::new();
    for s_i in s {
        *map.entry(s_i).or_insert(0) += 1;
    }
    for i in 0..26 {
        let c = (b'a' + i as u8) as char;
        match map.get(&c) {
            Some(&count) if count == 1 => {
                println!("{}", c);
                return;
            }
            Some(_) | None => {}
        }
    }
    println!("-1");
}
