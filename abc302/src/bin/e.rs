use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    };
    let mut count = n;
    let mut edges = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    u: Usize1,
                    v: Usize1,
                }
                edges[u].insert(v);
                edges[v].insert(u);
                if edges[u].len() == 1 {
                    count -= 1;
                }
                if edges[v].len() == 1 {
                    count -= 1;
                }
            }
            2 => {
                input! {
                    v: Usize1,
                }

                if !edges[v].is_empty() {
                    let us = edges[v].iter().cloned().collect::<Vec<_>>();
                    for u in us {
                        edges[u].remove(&v);
                        if edges[u].is_empty() {
                            count += 1;
                        }
                    }
                    edges[v].clear();
                    count += 1;
                }
            }
            _ => unreachable!(),
        }
        println!("{}", count);
    }
}
