use dsu::*;
use rand::{thread_rng, Rng};
use std::{
    collections::HashSet,
    time::{Duration, Instant},
};

use proconio::input;

fn get_edges(n: usize, c: &[Vec<usize>]) -> Vec<(usize, usize)> {
    let mut edges = HashSet::new();
    for i in 0..n {
        for j in 0..n {
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..n as i64).contains(&nr) || !(0..n as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if c[i][j] == 0 || c[nr][nc] == 0 || c[i][j] == c[nr][nc] {
                    continue;
                }
                let u = c[i][j] - 1;
                let v = c[nr][nc] - 1;
                edges.insert((u.min(v), u.max(v)));
            }
        }
    }
    edges.into_iter().collect()
}

fn adjacency_list(n: usize, uv: &[(usize, usize)]) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn calc_score_dfs(visited: &mut Vec<bool>, edges: &[Vec<usize>], ans: &[usize], u: usize) {
    visited[u] = true;
    for v in edges[u].iter().copied() {
        if ans[v] == ans[u] && !visited[v] {
            calc_score_dfs(visited, edges, ans, v);
        }
    }
}

fn calc_score(l: usize, ab: &[(usize, usize)], edges: &[Vec<usize>], ans: &[usize]) -> f64 {
    let k = ab.len();
    let mut visited = vec![false; k];
    for i in 0..l {
        match ans.iter().position(|ans_i| ans_i == &i) {
            None => return 0_f64,
            Some(u) => calc_score_dfs(&mut visited, &edges, &ans, u),
        }
    }
    if visited.iter().any(|b| !b) {
        return 0_f64;
    }

    let mut p = vec![0_usize; l];
    let mut q = vec![0_usize; l];
    for (i, ans_i) in ans.iter().copied().enumerate() {
        let (a, b) = ab[i];
        p[ans_i] += a;
        q[ans_i] += b;
    }
    (p.iter().copied().min().unwrap() as f64 / p.iter().copied().max().unwrap() as f64)
        .min(q.iter().copied().min().unwrap() as f64 / q.iter().copied().max().unwrap() as f64)
}

fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        ab: [(usize, usize); k],
        c: [[usize; n]; n],
    };

    let uv = get_edges(n, &c);
    let edges = adjacency_list(k, &uv);

    // 貪欲法
    let mut dsu = Dsu::new(k);
    for _ in 0..k - l {
        let mut min_size = 1_usize << 60;
        let mut min_vertex = (k, k);
        for u in 0..k {
            for v in edges[u].iter().copied() {
                if dsu.same(u, v) {
                    continue;
                }
                let size_u = dsu.size(u);
                let size_v = dsu.size(v);
                if size_u + size_v < min_size {
                    min_size = size_u + size_v;
                    min_vertex = (u, v);
                }
            }
        }
        let (u, v) = min_vertex;
        dsu.merge(u, v);
    }
    let mut ans = vec![l; k];
    for (i, group) in dsu.groups().into_iter().enumerate() {
        for g in group {
            ans[g] = i;
        }
    }

    // 山登り法
    let mut score = calc_score(l, &ab, &edges, &ans);
    let instant = Instant::now();
    let duration = Duration::new(0, 950_000_000);
    let mut rng = thread_rng();

    while instant.elapsed() < duration {
        let (u, x) = loop {
            let u = rng.gen::<usize>() % k;
            let x = ans[edges[u][rng.gen::<usize>() % edges[u].len()]];
            if ans[u] != x {
                break (u, x);
            }
        };
        let bak = ans[u];
        ans[u] = x;

        let new_score = calc_score(l, &ab, &edges, &ans);

        if score < new_score {
            score = new_score;
        } else {
            ans[u] = bak;
        }
    }

    // 出力
    for i in 0..k {
        println!("{}", ans[i] + 1);
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
