use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        abv: [(usize, usize); m],
        cdv: [(usize, usize); k],
    };
    let mut cv = T::new(n);
    for &(a, b) in abv.iter() {
        cv.unite(a - 1, b - 1);
    }

    let mut fv = vec![vec![]; n];
    for &(a, b) in abv.iter() {
        fv[a - 1].push(b - 1);
        fv[b - 1].push(a - 1);
    }

    let mut bv = vec![vec![]; n];
    for &(c, d) in cdv.iter() {
        bv[c - 1].push(d - 1);
        bv[d - 1].push(c - 1);
    }

    for i in 0..n {
        let r = cv.root(i);
        let a = cv.size(i)
            - fv[i].len()
            - 1
            - bv[i]
                .iter()
                .filter(|&&b| r == cv.root(b))
                .fold(0, |acc, _| acc + 1);
        println!("{}{}", a, if i == n - 1 { "\n" } else { " " });
    }
}

struct T {
    p: Vec<usize>,
    s: Vec<usize>,
}

impl T {
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
