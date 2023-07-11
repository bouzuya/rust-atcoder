use self::union_find::UnionFind;
use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn is_cycle(root: usize, e: &Vec<Vec<usize>>) -> bool {
    let mut is_cycle = false;
    let mut set = std::collections::BTreeSet::new();
    set.insert(root);
    let mut stack = vec![];
    stack.push((root, root));
    while let Some((p, u)) = stack.pop() {
        for &v in e[u].iter() {
            if v == p {
                continue;
            }
            if set.insert(v) {
                stack.push((u, v));
            } else {
                is_cycle = true;
                break;
            }
        }
        if is_cycle {
            break;
        }
    }
    is_cycle
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let e = adjacency_list(n, &uv);

    let mut uf = UnionFind::new(n);
    for &(u_i, v_i) in uv.iter() {
        uf.unite(u_i, v_i);
    }

    let mut roots = std::collections::BTreeSet::new();
    for i in 0..n {
        roots.insert(uf.root(i));
    }

    let mut count = 0;
    for &root in roots.iter() {
        if !is_cycle(root, &e) {
            count += 1;
        }
    }

    let ans = count;
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
