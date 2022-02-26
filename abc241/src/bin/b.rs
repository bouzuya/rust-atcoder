use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };
    let mut map = HashMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1;
    }

    for b_i in b {
        match map.get_mut(&b_i) {
            Some(count) => {
                if *count == 0 {
                    println!("No");
                    return;
                }
                *count -= 1;
            }
            None => {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
