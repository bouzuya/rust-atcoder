use proconio::{input, marker::Usize1};

// 重み付きUnionFind
struct WeightedUnionFind {
    parent: Vec<usize>,
    weight: Vec<i64>,
}

impl WeightedUnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            weight: vec![0; n],
        }
    }

    fn merge(&mut self, a: usize, b: usize, w: i64) -> bool {
        let (ra, rb) = (self.root(a), self.root(b));
        if ra == rb {
            return self.weight[b] - self.weight[a] == w;
        }
        self.parent[ra] = rb;
        self.weight[ra] = self.weight[b] - self.weight[a] - w;
        true
    }

    fn root(&mut self, a: usize) -> usize {
        if self.parent[a] == a {
            return a;
        }
        let p = self.parent[a];
        let r = self.root(p);
        self.parent[a] = r;
        self.weight[a] += self.weight[p];
        r
    }
}

fn main() {
    input! {
        _n: usize,
        q: usize,
        abd: [(Usize1, Usize1, i64); q],
    };

    let mut dsu = WeightedUnionFind::new(2 * 100_000_usize + 1);
    for (i, (a, b, d)) in abd.into_iter().enumerate() {
        if dsu.merge(a, b, d) {
            println!("{}", i + 1);
        }
    }
}
