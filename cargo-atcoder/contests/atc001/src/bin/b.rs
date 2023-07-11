use self::union_find::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        pab: [(usize, usize, usize); q],
    };
    let mut uf = UnionFind::new(n);
    for &(p_i, a_i, b_i) in pab.iter() {
        match p_i {
            0 => uf.unite(a_i, b_i),
            1 => println!(
                "{}",
                if uf.root(a_i) == uf.root(b_i) {
                    "Yes"
                } else {
                    "No"
                }
            ),
            _ => unreachable!(),
        }
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
