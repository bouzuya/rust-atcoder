use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        k: usize,
        l: usize,
        pq: [(Usize1, Usize1); k],
        rs: [(Usize1, Usize1); l],
    };
    let mut uf1 = UnionFind::new(n);
    for &(p_i, q_i) in pq.iter() {
        uf1.unite(p_i, q_i);
    }
    let mut uf2 = UnionFind::new(n);
    for &(r_i, s_i) in rs.iter() {
        uf2.unite(r_i, s_i);
    }
    let mut map = std::collections::BTreeMap::new();
    for i in 0..n {
        *map.entry((uf1.root(i), uf2.root(i))).or_insert(0) += 1;
    }
    for i in 0..n {
        print!(
            "{}{}",
            map.get(&(uf1.root(i), uf2.root(i))).unwrap_or(&0),
            if i == n - 1 { "\n" } else { " " }
        );
    }
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
