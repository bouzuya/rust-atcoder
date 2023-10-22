use dsu::*;
use im_rc::{HashMap, HashSet};
use proconio::{input, marker::Usize1};

fn depth_dfs(edges: &[Vec<usize>], depth: &mut Vec<usize>, u: usize, p: usize, l: usize) {
    depth[u] = l;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        depth_dfs(edges, depth, v, u, l + 1);
    }
}

fn depth(e: &[Vec<usize>], root: usize) -> Vec<usize> {
    let mut res = vec![0; e.len()];
    depth_dfs(&e, &mut res, root, root, 0);
    res
}

fn parent_dfs(edges: &[Vec<usize>], parent: &mut Vec<usize>, u: usize, p: usize) {
    parent[u] = p;
    for v in edges[u].iter().copied() {
        if v == p {
            continue;
        }
        parent_dfs(edges, parent, v, u);
    }
}

fn parent(e: &[Vec<usize>], root: usize) -> Vec<usize> {
    let mut res = vec![e.len(); e.len()];
    parent_dfs(&e, &mut res, root, e.len());
    res
}

fn parent_doubling(p: &[usize]) -> Vec<Vec<usize>> {
    let n = p.len();
    let k_len = p.len().next_power_of_two().trailing_zeros() as usize;
    let mut res = vec![vec![n; n]; k_len];
    for (i, p_i) in p.iter().copied().enumerate() {
        res[0][i] = p_i;
    }
    for k in 0..k_len - 1 {
        for i in 0..p.len() {
            if res[k][i] == n {
                res[k + 1][i] = n;
            } else {
                res[k + 1][i] = res[k][res[k][i]];
            }
        }
    }
    res
}

fn lca_by_doubling(depth: &[usize], parent: &[Vec<usize>], u: usize, v: usize) -> usize {
    let k_len = parent.len();
    // u と v の深さを揃える
    let (mut u, mut v) = if depth[u] > depth[v] { (v, u) } else { (u, v) };
    for k in 0..k_len {
        if (((depth[v] - depth[u]) >> k) & 1) == 1 {
            v = parent[k][v];
        }
    }
    if u == v {
        return u;
    }
    // 二分探索する
    for k in (0..k_len).rev() {
        if parent[k][u] != parent[k][v] {
            u = parent[k][u];
            v = parent[k][v];
        }
    }
    assert_eq!(parent[0][u], parent[0][v]);
    parent[0][u]
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        q: usize,
        uv: [(Usize1, Usize1); q],
    }

    let inf = 1_usize << 60;

    let mut dsu = Dsu::new(n);
    let mut edges1 = vec![vec![]; n];
    let mut edges2 = vec![vec![]; n];
    let mut set = HashSet::new();
    for (a, b) in ab {
        if dsu.same(a, b) {
            edges2[a].push(b);
            edges2[b].push(a);
            set.insert(a);
            set.insert(b);
        } else {
            dsu.merge(a, b);
            edges1[a].push(b);
            edges1[b].push(a);
        }
    }
    let mut edges_all = edges1.clone();
    for (i, edges2_i) in edges2.iter().enumerate() {
        for j in edges2_i.iter().copied() {
            edges_all[i].push(j);
        }
    }

    let depth = depth(&edges1, 0);
    let parent = parent(&edges1, 0);
    let parent = parent_doubling(&parent);

    let mut dist1 = HashMap::new();
    for u in set.iter().copied() {
        let m = dist1.entry(u).or_insert_with(HashMap::new);
        for v in set.iter().copied() {
            let lca = lca_by_doubling(&depth, &parent, u, v);
            let l = depth[u] + depth[v] - depth[lca] * 2;
            m.insert(v, l);
        }
    }
    for (u, vs) in edges2.iter().enumerate() {
        let m = dist1.entry(u).or_insert_with(HashMap::new);
        for v in vs.iter().copied() {
            let entry = m.entry(v).or_insert(inf);
            *entry = (*entry).min(1);
        }
    }
    for k in set.iter() {
        for i in set.iter() {
            for j in set.iter() {
                let d1 = dist1[i][k];
                let d2 = dist1[k][j];
                let entry = dist1[i].entry(*j).or_insert(inf);
                *entry = (*entry).min(d1 + d2);
            }
        }
    }

    let mut dist2 = HashMap::new();
    for u in set.iter().copied() {
        let dist = dist2.entry(u).or_insert_with(HashMap::new);
        for v in 0..n {
            let lca = lca_by_doubling(&depth, &parent, u, v);
            let l = depth[u] + depth[v] - depth[lca] * 2;
            dist.insert(v, l);
        }
    }

    for (u, v) in uv {
        let lca = lca_by_doubling(&depth, &parent, u, v);
        let mut ans = depth[u] + depth[v] - depth[lca] * 2;
        for i in set.iter() {
            for j in set.iter() {
                ans = ans.min(dist2[i][&u] + dist1[i][j] + dist2[j][&v]);
            }
        }
        println!("{}", ans);
    }
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
