// WA
use std::collections::BinaryHeap;

use dsu::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut d: [usize; n],
        ab: [(Usize1, Usize1); m],
    };

    if d.iter().copied().filter(|d_i| d_i == &1).count() > n - 1 {
        println!("-1");
        return;
    }

    let mut dsu = Dsu::new(n);
    for (a, b) in ab {
        if dsu.same(a, b) {
            println!("-1");
            return;
        }
        dsu.merge(a, b);
        if d[a] == 0 || d[b] == 0 {
            println!("-1");
            return;
        }
        d[a] -= 1;
        d[b] -= 1;
    }

    let mut pq0 = BinaryHeap::new();
    let mut pqx = BinaryHeap::new();
    for (i, d_i) in d.iter().copied().enumerate() {
        if d_i == 0 {
            continue;
        }
        if dsu.same(i, 0) {
            pq0.push((d_i, i));
        } else {
            pqx.push((d_i, i));
        }
    }

    let mut ans = vec![];
    while let Some((d_i, i)) = pq0.pop() {
        if let Some((d_j, j)) = pqx.pop() {
            if d_i - 1 > 0 {
                pq0.push((d_i - 1, i));
            }
            if d_j - 1 > 0 {
                pq0.push((d_j - 1, j));
            }
            ans.push((i, j));
            dsu.merge(i, j);
        } else {
            println!("-1");
            return;
        }
    }

    for u in 0..n {
        if !dsu.same(0, u) {
            println!("-1");
            return;
        }
    }

    for (u, v) in ans {
        println!("{} {}", u + 1, v + 1);
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
