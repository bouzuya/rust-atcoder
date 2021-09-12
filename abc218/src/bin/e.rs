use dsu::*;
use proconio::{input, marker::Usize1};

fn kruskal(n: usize, e: &mut Vec<(usize, usize, u64)>) -> u64 {
    e.sort_by_key(|&(_, _, w)| w);
    let mut dsu = Dsu::new(n);
    let mut sum = 0_u64;
    for (u_i, v_i, w_i) in e.iter().copied() {
        if !dsu.same(u_i, v_i) {
            dsu.merge(u_i, v_i);
            sum += w_i;
        }
    }
    sum
}

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(Usize1, Usize1, i64); m],
    };

    let mut ans = 0_u64;

    let mut dsu = Dsu::new(n);
    let mut e = vec![];
    for (a_i, b_i, c_i) in abc {
        // 自己ループは取り除く
        if a_i == b_i {
            if c_i > 0 {
                ans += c_i as u64;
            }
            continue;
        }

        // 負の辺は取り除く
        if c_i <= 0 {
            dsu.merge(a_i, b_i);
        } else {
            e.push((a_i, b_i, c_i as u64));
        }
    }

    let mut sum = 0_u64;
    let mut edge = vec![];
    for (a_i, b_i, c_i) in e {
        let u = dsu.leader(a_i);
        let v = dsu.leader(b_i);
        if u == v {
            ans += c_i;
        } else {
            edge.push((u, v, c_i));
            sum += c_i;
        }
    }

    let min_cost = kruskal(n, &mut edge);

    let ans = ans + sum - min_cost;
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
