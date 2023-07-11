use std::collections::HashMap;

use dsu::*;
use proconio::{input, marker::Usize1};
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abc: [(Usize1, Usize1, usize); m],
        uvw: [(Usize1, Usize1, usize); q],
    };
    let mut abci = abc
        .iter()
        .copied()
        .enumerate()
        .map(|(i, (a, b, c))| (a, b, c, i))
        .collect::<Vec<(usize, usize, usize, usize)>>();
    abci.sort_by_key(|&(_, _, c, _)| c);
    let mut dsu = Dsu::new(n);

    let mut uvwi = vec![];
    for (i, (u, v, w)) in uvw.iter().copied().enumerate() {
        let index = abci.lower_bound_by_key(&w, |&(_, _, c, _)| c);
        uvwi.push((u, v, index, i));
    }
    let mut map = HashMap::new();
    for (u, v, index, i) in uvwi {
        map.entry(index).or_insert(vec![]).push((u, v, i));
    }

    let mut ans = vec![false; q];
    for (j, (a, b, _, _)) in abci.iter().copied().enumerate() {
        if a == b {
            continue;
        }
        match map.get(&j) {
            Some(uvx) => {
                for (u, v, x) in uvx.iter().copied() {
                    ans[x] = !dsu.same(u, v);
                }
            }
            None => {}
        }
        if dsu.same(a, b) {
            continue;
        }
        dsu.merge(a, b);
    }

    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
