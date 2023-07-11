use std::collections::HashMap;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
    };
    let mut base = 0_usize;
    let mut map = HashMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        map.insert(i, a_i);
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
                map.clear();
            }
            2 => {
                input! {
                    i: Usize1,
                    x: usize,
                }
                *map.entry(i).or_insert(base) += x;
            }
            3 => {
                input! {
                    i: Usize1,
                }
                println!("{}", *map.get(&i).unwrap_or(&base));
            }
            _ => unreachable!(),
        }
    }
}
