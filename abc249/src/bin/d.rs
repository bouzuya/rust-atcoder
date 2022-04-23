use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut map = BTreeMap::new();
    for a_i in a {
        *map.entry(a_i).or_insert(0) += 1_usize;
    }

    let mut set = BTreeSet::new();
    for (&v, _) in map.iter() {
        for i in 1.. {
            if i * i > v {
                break;
            }
            if v % i == 0 {
                let a = i;
                let b = v / i;
                set.insert((a.min(b), a.max(b)));
            }
        }
    }

    let mut ans = 0_usize;
    for (a, b) in set {
        match (map.get(&(a * b)), map.get(&a), map.get(&b)) {
            (None, _, _) => continue,
            (Some(_), None, None) => continue,
            (Some(_), None, Some(_)) => continue,
            (Some(_), Some(_), None) => continue,
            (Some(c_ab), Some(&c_a), Some(&c_b)) => {
                if a != b {
                    ans += c_ab * c_a * c_b * 2;
                } else {
                    ans += c_ab * c_a * c_b;
                }
            }
        }
    }

    println!("{}", ans);
}
