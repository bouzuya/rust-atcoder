use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    };

    let mut base = 0_usize;
    let mut added = HashMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        added.insert(i + 1, a_i);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                base = x;
                added.clear();
            }
            2 => {
                input! {
                    i: usize,
                    x: usize,
                }
                *added.entry(i).or_insert(0) += x;
            }
            3 => {
                input! {
                    i: usize,
                }
                let ans = base + added.get(&i).unwrap_or(&0);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
    }
}
