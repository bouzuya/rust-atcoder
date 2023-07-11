use dsu::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        abcd: [(Usize1, char, Usize1, char); m],
    };

    let mut edges = vec![vec![]; 2 * n];
    for i in 0..n {
        let u = i;
        let v = i + n;
        edges[u].push(v);
        edges[v].push(u);
    }

    for (a, b, c, d) in abcd {
        let u = match b {
            'R' => a,
            'B' => a + n,
            _ => unreachable!(),
        };
        let v = match d {
            'R' => c,
            'B' => c + n,
            _ => unreachable!(),
        };
        edges[u].push(v);
        edges[v].push(u);
    }

    let mut dsu = Dsu::new(2 * n);
    for u in 0..2 * n {
        for v in edges[u].iter().copied() {
            dsu.merge(u, v);
        }
    }

    let mut count_all = 0_usize;
    let mut count_cycle = 0_usize;
    'outer: for group in dsu.groups() {
        count_all += 1;
        let count_v = group.len();
        let mut count_e = 0_usize;
        for g in group {
            count_e += edges[g].len();
            if edges[g].len() != 2 {
                continue 'outer;
            }
        }
        if count_e != count_v * 2 {
            break;
        }
        count_cycle += 1;
    }

    println!("{} {}", count_cycle, count_all - count_cycle);
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
