use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abyv: [(Usize1, Usize1, usize); m], // a -> b, y
        q: usize,
        vwv: [(Usize1, usize); q],
    };

    let mut dv = vec![];
    for (a, b, y) in abyv {
        dv.push((y, (0, a, b)));
    }
    for (i, &(v, y)) in vwv.iter().enumerate() {
        dv.push((y, (1, v, i)));
    }
    dv.sort_by(|a, b| b.cmp(&a));

    let mut av = vec![0_usize; q];
    let mut uf = UnionFind::new(n);
    for (_, d) in dv {
        match d {
            (0, a, b) => {
                uf.unite(a, b);
            }
            (1, v, i) => {
                let root = uf.root(v);
                av[i] = uf.size(root);
            }
            (_, _, _) => unreachable!(),
        }
    }

    for ans in av {
        println!("{}", ans);
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

        pub fn size(&mut self, x: usize) -> usize {
            let rx = self.root(x);
            self.s[rx]
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
