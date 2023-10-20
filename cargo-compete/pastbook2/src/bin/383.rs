use dsu::*;
use proconio::input;

fn dist((px_i, py_i): (i64, i64), (px_j, py_j): (i64, i64)) -> f64 {
    (((px_i - px_j).pow(2) + (py_i - py_j).pow(2)) as f64).sqrt()
}

fn kruskal(n: usize, e: &mut [(usize, usize, f64)]) -> f64 {
    e.sort_by(|(_, _, w1), (_, _, w2)| w1.partial_cmp(w2).unwrap());
    let mut dsu = Dsu::new(n);
    let mut sum = 0_f64;
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
        p: [(i64, i64); n],
        cr: [(i64, i64, i64); m],
    }

    let mut edges = vec![vec![]; n];
    for (i, p_i) in p.iter().copied().enumerate() {
        for (j, p_j) in p.iter().copied().enumerate() {
            if i == j {
                continue;
            }
            edges[i].push((j, dist(p_i, p_j)));
        }
    }

    let mut min = 1e18;
    for bits in 0..1 << m {
        let cr = cr
            .iter()
            .copied()
            .enumerate()
            .filter(|(i, _)| ((bits >> i) & 1) == 1)
            .map(|(_, (x, y, r))| (x, y, r))
            .collect::<Vec<_>>();
        let m = cr.len();
        let edges = {
            let mut e = vec![vec![]; n + m];
            for (i, edges_i) in edges.iter().enumerate() {
                e[i] = edges_i.clone();
            }
            for (i, p_i) in p.iter().copied().enumerate() {
                for (j, (cx_j, cy_j, r_j)) in cr.iter().copied().enumerate() {
                    let d = (dist(p_i, (cx_j, cy_j)) - r_j as f64).abs();
                    e[i].push((n + j, d));
                    e[n + j].push((i, d));
                }
            }
            for (i, (cx_i, cy_i, r_i)) in cr.iter().copied().enumerate() {
                for (j, (cx_j, cy_j, r_j)) in cr.iter().copied().enumerate() {
                    if i == j {
                        continue;
                    }
                    let d = dist((cx_i, cy_i), (cx_j, cy_j));
                    let r1 = (r_i + r_j) as f64;
                    let r2 = (r_i - r_j).abs() as f64;
                    let d = if r1 <= d {
                        d - (r_i + r_j) as f64
                    } else if r2 < d {
                        0.0
                    } else {
                        r2 - d
                    };
                    e[n + i].push((n + j, d));
                }
            }
            e
        };

        let mut edges = {
            let mut e = vec![];
            for (i, jd) in edges.into_iter().enumerate() {
                for (j, d) in jd {
                    e.push((i, j, d));
                }
            }
            e
        };

        let min_cost = kruskal(n + m, &mut edges);
        if min_cost < min {
            min = min_cost;
        }
    }

    println!("{}", min);
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    //! A Disjoint set union (DSU) with union by size and path compression.

    /// A Disjoint set union (DSU) with union by size and path compression.
    ///
    /// See: [Zvi Galil and Giuseppe F. Italiano, Data structures and algorithms for disjoint set union problems](https://core.ac.uk/download/pdf/161439519.pdf)
    ///
    /// In the following documentation, let $n$ be the size of the DSU.
    ///
    /// # Example
    ///
    /// ```
    /// use ac_library::Dsu;
    /// use proconio::{input, source::once::OnceSource};
    ///
    /// input! {
    ///     from OnceSource::from(
    ///         "5\n\
    ///          3\n\
    ///          0 1\n\
    ///          2 3\n\
    ///          3 4\n",
    ///     ),
    ///     n: usize,
    ///     abs: [(usize, usize)],
    /// }
    ///
    /// let mut dsu = Dsu::new(n);
    /// for (a, b) in abs {
    ///     dsu.merge(a, b);
    /// }
    ///
    /// assert!(dsu.same(0, 1));
    /// assert!(!dsu.same(1, 2));
    /// assert!(dsu.same(2, 4));
    ///
    /// assert_eq!(
    ///     dsu.groups()
    ///         .into_iter()
    ///         .map(|mut group| {
    ///             group.sort_unstable();
    ///             group
    ///         })
    ///         .collect::<Vec<_>>(),
    ///     [&[0, 1][..], &[2, 3, 4][..]],
    /// );
    /// ```
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        /// Creates a new `Dsu`.
        ///
        /// # Constraints
        ///
        /// - $0 \leq n \leq 10^8$
        ///
        /// # Complexity
        ///
        /// - $O(n)$
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }

        // `\textsc` does not work in KaTeX
        /// Performs the Uɴɪᴏɴ operation.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        /// - $0 \leq b < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraints are not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
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

        /// Returns whether the vertices $a$ and $b$ are in the same connected component.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        /// - $0 \leq b < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraint is not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }

        /// Performs the Fɪɴᴅ operation.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraint is not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }

        /// Returns the size of the connected component that contains the vertex $a$.
        ///
        /// # Constraints
        ///
        /// - $0 \leq a < n$
        ///
        /// # Panics
        ///
        /// Panics if the above constraint is not satisfied.
        ///
        /// # Complexity
        ///
        /// - $O(\alpha(n))$ amortized
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }

        /// Divides the graph into connected components.
        ///
        /// The result may not be ordered.
        ///
        /// # Complexity
        ///
        /// - $O(n)$
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
            assert!(d.same(0, 1));
            d.merge(1, 2);
            assert!(d.same(0, 2));
            assert_eq!(d.size(0), 3);
            assert!(!d.same(0, 3));
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
