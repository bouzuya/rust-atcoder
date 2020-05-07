use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn f(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    n * (n - 1) / 2
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let mut ans = vec![0_usize; ab.len() + 1];
    let mut uf = UnionFind::new(n);
    ans[0] = f(n);
    for (i, &(a, b)) in ab.iter().rev().enumerate() {
        let s_a = uf.size(a);
        let s_b = uf.size(b);
        let o = if uf.root(a) == uf.root(b) {
            f(s_a)
        } else {
            f(s_a) + f(s_b)
        };
        uf.unite(a, b);
        let s_ab = uf.size(a);
        ans[i + 1] = ans[i] + o - f(s_ab);
    }
    for &a in ans.iter().rev().skip(1) {
        println!("{}", a);
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
