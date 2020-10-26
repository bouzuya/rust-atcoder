use self::atcoder::dsu::Dsu;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; n],
        cd: [(Usize1, Usize1); m],
    };

    let mut dsu = Dsu::new(n);
    for &(c_i, d_i) in cd.iter() {
        dsu.merge(c_i, d_i);
    }

    for group in dsu.groups() {
        let mut sum_a = 0_i64;
        let mut sum_b = 0_i64;
        for u in group {
            sum_a += a[u];
            sum_b += b[u];
        }
        if sum_a != sum_b {
            println!("No");
            return;
        }
    }
    println!("Yes");
}

pub mod atcoder {
    pub mod dsu {
        // https://raw.githubusercontent.com/rust-lang-ja/ac-library-rs/ad148b05b07e3ac59cbc2987b8834742ef3d497a/src/dsu.rs
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
    }
}
