use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

macro_rules! chmin {
    ($e: expr, $v: expr) => {
        if $v < $e {
            $e = $v;
        }
    };
}

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn bfs(n: usize, inf: u64, e: &Vec<Vec<usize>>, s: usize) -> Vec<u64> {
    let mut d = vec![inf; n];
    let mut q = std::collections::VecDeque::new();
    d[s] = 0;
    q.push_back((0, s));
    while let Some((w_u, u)) = q.pop_front() {
        if w_u > d[u] {
            continue;
        }
        for &v in e[u].iter() {
            let w = w_u + 1;
            if w < d[v] {
                d[v] = w;
                q.push_back((w, v));
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
        s: Usize1,
        k: usize,
        t: [Usize1; k],
    };

    let e = adjacency_list(n, &uv);
    let inf = 1_000_000_000_000;
    let d_s = bfs(n, inf, &e, s);
    let mut d_t = vec![vec![inf; k]; k];
    for (i, &t_i) in t.iter().enumerate() {
        let d_t_i = bfs(n, inf, &e, t_i);
        for (j, &t_j) in t.iter().enumerate() {
            chmin!(d_t[i][j], d_t_i[t_j]);
            chmin!(d_t[j][i], d_t_i[t_j]);
        }
    }

    let mut et = vec![];
    for i in 0..k {
        for j in i + 1..k {
            et.push((i, j, d_t[i][j]));
        }
    }
    for i in 0..k {
        et.push((i, k, d_s[t[i]]));
    }
    et.sort_by_key(|&(_, _, w)| w);

    let mut ans = 0;
    let mut uf = UnionFind::new(k + 1);
    for &(u, v, w) in et.iter() {
        if uf.root(u) == uf.root(v) {
            continue;
        }
        uf.unite(u, v);
        ans += w;
    }
    println!("{}", ans + 1);
}

mod union_find {
    pub struct UnionFind {
        p: Vec<usize>,
        s: Vec<usize>,
    }

    impl UnionFind {
        pub fn new(n: usize) -> Self {
            let mut p = vec![0; n];
            for i in 0..n {
                p[i] = i;
            }
            let s = vec![1; n];
            Self { p, s }
        }

        pub fn root(&mut self, x: usize) -> usize {
            if self.p[x] == x {
                return x;
            } else {
                self.p[x] = self.root(self.p[x]);
                self.p[x]
            }
        }

        pub fn unite(&mut self, x: usize, y: usize) {
            let rx = self.root(x);
            let ry = self.root(y);
            if rx == ry {
                return;
            };
            if self.s[rx] >= self.s[ry] {
                self.s[rx] += self.s[ry];
                self.p[ry] = rx;
            } else {
                self.s[ry] += self.s[rx];
                self.p[rx] = ry;
            }
        }
    }
}
