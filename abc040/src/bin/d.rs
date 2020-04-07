use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

enum D {
    E { a: usize, b: usize, y: isize },
    Q { i: usize, v: usize, w: isize },
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abyv: [(Usize1, Usize1, isize); m],
        q: usize,
        vwv: [(Usize1, isize); q],
    };

    // 道路と人をまとめて、年の降順、年が同じとき人が前にくるよう並べる
    let mut dv: Vec<D> = abyv
        .iter()
        .map(|&(a, b, y)| D::E { a, b, y })
        .chain(vwv.iter().enumerate().map(|(i, &(v, w))| D::Q { i, v, w }))
        .collect();
    dv.sort_by_key(|d| match d {
        D::E { a: _, b: _, y } => (-y, 1),
        D::Q { i: _, v: _, w } => (-w, 0),
    });

    let mut av = vec![0_usize; q];
    let mut uf = UnionFind::new(n);
    for d in dv {
        match d {
            D::E { a, b, y: _ } => uf.unite(a, b),
            D::Q { i, v, w: _ } => av[i] = uf.size(v),
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
