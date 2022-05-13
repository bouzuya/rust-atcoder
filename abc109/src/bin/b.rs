use std::collections::BTreeSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        w: [Chars; n]
    };
    let mut used = BTreeSet::new();
    let mut prev: Option<Vec<char>> = None;
    for w_i in w {
        if let Some(ref w_p) = prev {
            let c_p = *w_p.last().unwrap();
            let c_i = *w_i.first().unwrap();
            if c_p != c_i {
                println!("No");
                return;
            }
        }
        if !used.insert(w_i.clone()) {
            println!("No");
            return;
        }
        prev = Some(w_i);
    }
    println!("Yes");
}
