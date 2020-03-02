use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abv: [(Usize1, Usize1); m],
        cdv: [(Usize1, Usize1); k],
    };

    let mut fcv = vec![0usize; n];
    let mut uf = UnionFind::new(n);
    for &(a, b) in abv.iter() {
        fcv[a] += 1;
        fcv[b] += 1;
        uf.unite(a, b);
    }

    let mut bv = vec![vec![]; n];
    for &(c, d) in cdv.iter() {
        bv[c].push(d);
        bv[d].push(c);
    }

    for i in 0..n {
        let a = uf.size(i)
            - fcv[i]
            - 1
            - bv[i]
                .iter()
                .filter(|&&b| uf.same(i, b))
                .fold(0, |acc, _| acc + 1);
        println!("{}{}", a, if i == n - 1 { "\n" } else { " " });
    }
}

struct UnionFind {
    p: Vec<usize>,
    s: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut p = vec![0; n];
        for i in 0..n {
            p[i] = i;
        }
        let s = vec![1; n];
        Self { p, s }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.p[x] == x {
            return x;
        } else {
            self.p[x] = self.root(self.p[x]);
            self.p[x]
        }
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        rx == ry
    }

    fn size(&mut self, x: usize) -> usize {
        let rx = self.root(x);
        self.s[rx]
    }

    fn unite(&mut self, x: usize, y: usize) {
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
