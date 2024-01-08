use std::collections::{BTreeMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        d: usize,
        a: [usize; n],
    };
    if (k - 1) * d >= n {
        println!("-1");
        return;
    }

    let mut map = BTreeMap::new();
    let mut ans = vec![];
    // [l, r)
    let mut l = 0_usize;
    let mut r = n - (k - 1) * d;
    for i in l..r {
        if i >= n {
            break;
        }
        map.entry(a[i]).or_insert_with(VecDeque::new).push_back(i);
    }
    for _ in 0..k {
        let (a_i, mut is) = map.pop_first().unwrap();
        let min_pos = is.pop_front().unwrap();
        if !is.is_empty() {
            map.insert(a_i, is);
        }
        ans.push(a_i);

        for i in r..r + d {
            if i >= n {
                break;
            }
            map.entry(a[i]).or_insert_with(VecDeque::new).push_back(i);
        }
        for i in l..min_pos + d {
            if i >= n {
                break;
            }
            if i == min_pos {
                continue;
            }
            let is = map.get_mut(&a[i]).unwrap();
            is.pop_front();
            if is.is_empty() {
                map.remove(&a[i]);
            }
        }
        l = min_pos + d;
        r += d;
    }
    if ans.len() != k {
        println!("-1");
        return;
    }

    for a in ans {
        println!("{}", a);
    }
}
