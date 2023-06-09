use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut edges = HashSet::new();
    for (u, v) in uv {
        if !(0..n).contains(&u) || !(0..n).contains(&v) {
            println!("No");
            return;
        }
        if u == v {
            println!("No");
            return;
        }
        if !edges.insert((u.min(v), u.max(v))) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
