use std::collections::HashSet;

use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        q: usize,
        txy: [(usize, Usize1, Usize1); q],
    };
    let mut edges = HashSet::new();
    for (a, b) in ab.iter().copied() {
        edges.insert((a.min(b), a.max(b)));
    }
    for (t, x, y) in txy.iter().copied() {
        match t {
            1 => {
                edges.insert((x.min(y), x.max(y)));
            }
            2 => {
                edges.remove(&(x.min(y), x.max(y)));
            }
            3 => {
                // do nothing
            }
            _ => unreachable!(),
        }
    }

    let mut dsu = Dsu::new(n);
    for (a, b) in edges.iter().copied() {
        dsu.merge(a, b);
    }

    let mut ans = vec![];
    for (t, x, y) in txy.iter().copied().rev() {
        match t {
            1 => {
                edges.remove(&(x.min(y), x.max(y)));
                // rebuild dsu
                dsu = Dsu::new(n);
                for (a, b) in edges.iter().copied() {
                    dsu.merge(a, b);
                }
            }
            2 => {
                edges.insert((x.min(y), x.max(y)));
                dsu.merge(x, y);
            }
            3 => {
                ans.push(dsu.same(x, y));
            }
            _ => unreachable!(),
        }
    }
    for a in ans.into_iter().rev() {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
