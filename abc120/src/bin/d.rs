use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let ans_0 = n * (n - 1) / 2;
    let ans = std::iter::once(ans_0)
        .chain(
            ab.iter()
                .rev()
                .scan((ans_0, UnionFind::new(n)), |acc, &(a, b)| {
                    let ans = &mut acc.0;
                    let uf = &mut acc.1;
                    let s_a = uf.size(a);
                    let s_b = uf.size(b);
                    let is_same = uf.root(a) == uf.root(b);
                    uf.unite(a, b);
                    *ans -= if is_same { 0 } else { s_a * s_b };
                    Some(*ans)
                }),
        )
        .collect::<Vec<_>>();
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
