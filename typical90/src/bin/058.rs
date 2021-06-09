use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };
    let m = 100_000;
    let mut v = vec![0; m + 1];
    v[0] = n;
    for i in 0..m {
        let x = v[i];
        let y = x
            .to_string()
            .bytes()
            .fold(0, |acc, b| acc + (b - b'0') as usize);
        let z = (x + y) % m;
        v[i + 1] = z;
    }

    let mut set = BTreeSet::new();
    let mut start = 0;
    let mut cycle = 0;
    for (i, &v_i) in v.iter().enumerate() {
        if !set.insert(v_i) {
            start = v.iter().position(|&v_j| v_j == v_i).unwrap();
            cycle = i - start;
            break;
        }
    }
    let ans = if k < start {
        v[k]
    } else {
        v[start + (k - start) % cycle]
    };
    println!("{}", ans);
}
