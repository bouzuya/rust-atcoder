use dsu::*;
use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Usize1};

fn adjacency_list(
    n: usize,
    uvw: &[(usize, usize, u64)],
) -> (Vec<HashMap<usize, u64>>, Vec<Vec<(usize, u64)>>) {
    let mut e_in = vec![HashMap::new(); n];
    let mut e_out = vec![vec![]; n];
    for (u, v, w) in uvw.iter().copied() {
        e_out[u].push((v, w));
        e_in[v].insert(u, w);
    }
    (e_in, e_out)
}

fn main() {
    input! {
        n: usize,
        x: [Usize1; n],
        c: [u64; n],
    };

    let edges = x
        .iter()
        .copied()
        .zip(c.iter().copied())
        .enumerate()
        .map(|(i, (x_i, c_i))| (i, x_i, c_i))
        .collect::<Vec<(usize, usize, u64)>>();

    let (mut e_in, e_out) = adjacency_list(n, &edges);

    let mut used = vec![false; n];
    let mut deque = VecDeque::new();
    for v in 0..n {
        if e_in[v].is_empty() {
            deque.push_back(v);
            used[v] = true;
        }
    }
    while let Some(u) = deque.pop_front() {
        for (v, _) in e_out[u].iter().copied() {
            e_in[v].remove(&u);
            if e_in[v].is_empty() {
                deque.push_back(v);
                used[v] = true;
            }
        }
    }

    let mut dsu = Dsu::new(n);
    for u in 0..n {
        for (&v, _) in e_in[u].iter() {
            dsu.merge(u, v);
        }
    }
    let mut ans = 0_u64;
    for group in dsu.groups() {
        let mut min = None;
        for u in group {
            min = match (min, e_in[u].iter().map(|(_, &w)| w).min()) {
                (None, None) => None,
                (None, Some(v)) => Some(v),
                (Some(v), None) => Some(v),
                (Some(v1), Some(v2)) => Some(v1.min(v2)),
            }
        }
        ans += min.unwrap_or_default();
    }
    println!("{}", ans);
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
