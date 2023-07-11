use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        d: [i64; n],
        m: usize,
        t: [i64; m],
    };
    let mut map = BTreeMap::new();
    for d_i in d {
        *map.entry(d_i).or_insert(0) += 1;
    }
    for t_i in t {
        match map.get(&t_i) {
            Some(&count) => {
                if count <= 0 {
                    println!("NO");
                    return;
                }
                *map.entry(t_i).or_insert(0) -= 1;
            }
            None => {
                println!("NO");
                return;
            }
        }
    }

    println!("YES");
}
