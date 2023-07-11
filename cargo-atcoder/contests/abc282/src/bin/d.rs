use dsu::*;
use proconio::{input, marker::Usize1};

fn f(edges: &[Vec<usize>], component_index: &[usize], color: &mut Vec<usize>, u: usize) -> bool {
    let cur = color[component_index[u]];
    let next = if cur == 1 { 0 } else { 1 };
    for v in edges[u].iter().copied() {
        if color[component_index[v]] == 2 {
            color[component_index[v]] = next;
            if !f(edges, component_index, color, v) {
                return false;
            }
        } else if color[component_index[v]] != next {
            return false;
        }
    }
    true
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(Usize1, Usize1); m],
    };

    let mut edges = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        edges[u].push(v);
        edges[v].push(u);
    }

    let mut dsu = Dsu::new(n);
    for (u, v) in uv.iter().copied() {
        dsu.merge(u, v);
    }

    let mut count_edges = vec![0; n];
    for (u, _) in uv.iter().copied() {
        count_edges[dsu.leader(u)] += 1;
    }

    let mut ans = 0_usize;

    let mut component_index = vec![n; n];
    let mut sum_component_size = 0_usize;
    let mut components = vec![];
    for group in dsu.groups() {
        let component_size = group.len();
        let start = group[0];
        for (i, g) in group.iter().copied().enumerate() {
            component_index[g] = i;
        }
        let mut color = vec![2; component_size];
        color[component_index[start]] = 0;
        if f(&edges, &component_index, &mut color, start) {
            components.push(component_size);
            sum_component_size += component_size;

            let mut count_zero = 0_usize;
            for g in group.iter().copied() {
                if color[component_index[g]] == 0 {
                    count_zero += 1;
                }
            }
            let count_one = component_size - count_zero;
            ans += count_zero * count_one - count_edges[dsu.leader(start)];
        } else {
            println!("0");
            return;
        }
    }

    let mut a = 0_usize;
    for c in components {
        a += (sum_component_size - c) * c;
    }
    a /= 2;

    ans += a;

    println!("{}", ans);
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod dsu {
    /// Implement (union by size) + (path compression)
    /// Reference:
    /// Zvi Galil and Giuseppe F. Italiano,
    /// Data structures and algorithms for disjoint set union problems
    pub struct Dsu {
        n: usize,
        // root node: -1 * component size
        // otherwise: parent
        parent_or_size: Vec<i32>,
    }

    impl Dsu {
        // 0 <= size <= 10^8 is constrained.
        pub fn new(size: usize) -> Self {
            Self {
                n: size,
                parent_or_size: vec![-1; size],
            }
        }
        pub fn merge(&mut self, a: usize, b: usize) -> usize {
            assert!(a < self.n);
            assert!(b < self.n);
            let (mut x, mut y) = (self.leader(a), self.leader(b));
            if x == y {
                return x;
            }
            if -self.parent_or_size[x] < -self.parent_or_size[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.parent_or_size[x] += self.parent_or_size[y];
            self.parent_or_size[y] = x as i32;
            x
        }

        pub fn same(&mut self, a: usize, b: usize) -> bool {
            assert!(a < self.n);
            assert!(b < self.n);
            self.leader(a) == self.leader(b)
        }
        pub fn leader(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            if self.parent_or_size[a] < 0 {
                return a;
            }
            self.parent_or_size[a] = self.leader(self.parent_or_size[a] as usize) as i32;
            self.parent_or_size[a] as usize
        }
        pub fn size(&mut self, a: usize) -> usize {
            assert!(a < self.n);
            let x = self.leader(a);
            -self.parent_or_size[x] as usize
        }
        pub fn groups(&mut self) -> Vec<Vec<usize>> {
            let mut leader_buf = vec![0; self.n];
            let mut group_size = vec![0; self.n];
            for i in 0..self.n {
                leader_buf[i] = self.leader(i);
                group_size[leader_buf[i]] += 1;
            }
            let mut result = vec![Vec::new(); self.n];
            for i in 0..self.n {
                result[i].reserve(group_size[i]);
            }
            for i in 0..self.n {
                result[leader_buf[i]].push(i);
            }
            result
                .into_iter()
                .filter(|x| !x.is_empty())
                .collect::<Vec<Vec<usize>>>()
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn dsu_works() {
            let mut d = Dsu::new(4);
            d.merge(0, 1);
            assert_eq!(d.same(0, 1), true);
            d.merge(1, 2);
            assert_eq!(d.same(0, 2), true);
            assert_eq!(d.size(0), 3);
            assert_eq!(d.same(0, 3), false);
            assert_eq!(d.groups(), vec![vec![0, 1, 2], vec![3]]);
        }
    }
}
