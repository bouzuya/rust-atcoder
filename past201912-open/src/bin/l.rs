use self::union_find::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        nxyc: [(i64, i64, i64); n],
        mxyc: [(i64, i64, i64); m],
    };

    let mut ans = 1_000_000_000_000_f64;
    for bits in 0..1 << m {
        let mut xyc = nxyc.clone();
        for i in 0..m {
            if (bits >> i) & 1 == 1 {
                xyc.push(mxyc[i]);
            }
        }

        let mut e = vec![];
        for (u, &(u_x, u_y, u_c)) in xyc.iter().enumerate() {
            for (v, &(v_x, v_y, v_c)) in xyc.iter().enumerate() {
                if u <= v {
                    continue;
                }
                let c = (((u_x - v_x).pow(2) + (u_y - v_y).pow(2)) as f64).sqrt()
                    * if u_c == v_c { 1_f64 } else { 10_f64 };
                e.push((c, u, v));
            }
        }
        e.sort_by(|(c1, _, _), (c2, _, _)| c1.partial_cmp(c2).unwrap());
        let mut sum = 0_f64;
        let mut uf = UnionFind::new(xyc.len());
        for &(c, u, v) in e.iter() {
            if uf.root(u) == uf.root(v) {
                continue;
            }
            uf.unite(u, v);
            sum += c;
        }
        ans = if sum < ans { sum } else { ans };
    }
    println!("{}", ans);
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
