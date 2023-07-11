use dsu::*;
use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };

    let mut dsu = Dsu::new(n);
    for &(a_i, b_i) in ab.iter() {
        dsu.merge(a_i, b_i);
    }

    let e = adjacency_list(n, &ab);

    let mut ans = 1;
    for group in dsu.groups() {
        let size = group.len();
        let color_pattern = vec![vec![1, 2], vec![0, 2], vec![0, 1]];
        let mut count = 0_i64;
        for bits in 0..1 << (size - 1) {
            let is = (0..size - 1)
                .map(|i| (bits >> i) & 1)
                .collect::<Vec<usize>>();
            let mut js = vec![2; n];
            for (i, &g) in group.iter().enumerate() {
                if i > 0 {
                    js[g] = is[i - 1];
                }
            }

            let mut color = vec![3; n];
            let mut q = VecDeque::new();
            q.push_back((group[0], group[0]));
            color[group[0]] = 0;
            while let Some((u, p)) = q.pop_front() {
                for &v in e[u].iter() {
                    if v == p {
                        continue;
                    }
                    if color[v] == 3 {
                        color[v] = color_pattern[color[u]][js[v]];
                        q.push_back((v, u));
                    }
                }
            }
            let mut is_ok = true;
            for &u in group.iter() {
                for &v in e[u].iter() {
                    if color[u] == color[v] {
                        is_ok = false;
                    }
                }
            }
            if is_ok {
                count += 1;
            }
        }
        ans *= count * 3;
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
