use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String; n],
    };
    let mut map = HashMap::new();
    for s_i in s {
        match map.get_mut(&s_i) {
            None => {
                map.insert(s_i.clone(), 1);
                println!("{}", s_i);
            }
            Some(count) => {
                println!("{}({})", s_i, count);
                *count += 1;
            }
        }
    }
}
