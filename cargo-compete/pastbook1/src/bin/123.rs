use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut follows = vec![HashSet::new(); n];
    let mut followers = vec![HashSet::new(); n];
    for _ in 0..q {
        input! {
            t: usize,
            a: Usize1,
        }
        match t {
            1 => {
                input! {
                    b: Usize1,
                }
                follows[a].insert(b);
                followers[b].insert(a);
            }
            2 => {
                for b in followers[a].clone() {
                    follows[a].insert(b);
                    followers[b].insert(a);
                }
            }
            3 => {
                let mut bs = HashSet::new();
                for x in follows[a].clone() {
                    for b in follows[x].clone() {
                        bs.insert(b);
                    }
                }
                for b in bs {
                    follows[a].insert(b);
                    followers[b].insert(a);
                }
            }
            _ => unreachable!(),
        }
    }

    for i in 0..n {
        for j in 0..n {
            print!(
                "{}",
                if i != j && follows[i].contains(&j) {
                    "Y"
                } else {
                    "N"
                }
            );
        }
        println!();
    }
}
