use dsu::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut d: [usize; n],
        ab: [(Usize1, Usize1); m],
    };

    if d.iter().copied().sum::<usize>() != 2 * (n - 1) {
        println!("-1");
        return;
    }

    let mut dsu = Dsu::new(n);
    for (a, b) in ab {
        if d[a] == 0 || d[b] == 0 {
            println!("-1");
            return;
        }
        d[a] -= 1;
        d[b] -= 1;
        if dsu.same(a, b) {
            println!("-1");
            return;
        }
        dsu.merge(a, b);
    }

    #[derive(Clone, Debug)]
    struct C {
        count: usize,
        index: Vec<usize>,
    }
    let mut cs = dsu
        .groups()
        .into_iter()
        .map(|group| C {
            count: group.iter().copied().map(|i| d[i]).sum(),
            index: group
                .into_iter()
                .filter(|&i| d[i] > 0)
                .collect::<Vec<usize>>(),
        })
        .collect::<Vec<C>>();
    let mut c1 = vec![];
    let mut cx = vec![];
    for (i, C { count, .. }) in cs.iter().enumerate() {
        match &count {
            0 => continue,
            1 => c1.push(i),
            _ => cx.push(i),
        }
    }

    let mut ans = vec![];
    while let Some(i) = c1.pop() {
        if let Some(j) = cx.pop() {
            assert!(cs[i].count == 1);
            assert!(cs[j].count >= 1);
            cs[i].count -= 1;
            cs[j].count -= 1;
            match (cs[i].index.pop(), cs[j].index.pop()) {
                (Some(index_i), Some(index_j)) => {
                    ans.push((index_i, index_j));
                    assert!(d[index_i] == 1);
                    assert!(d[index_j] >= 1);
                    d[index_i] -= 1;
                    d[index_j] -= 1;
                    if d[index_j] > 0 {
                        cs[j].index.push(index_j);
                    }
                    assert!(!dsu.same(index_i, index_j));
                    dsu.merge(index_i, index_j);
                }
                _ => {
                    println!("-1");
                    return;
                }
            }
            match cs[j].count {
                0 => continue,
                1 => c1.push(j),
                _ => cx.push(j),
            }
        } else {
            c1.push(i);
            break;
        }
    }

    if cs.iter().cloned().filter(|c| c.count > 1).count() > 0 {
        println!("-1");
        return;
    }
    let c1 = cs
        .iter()
        .enumerate()
        .filter(|(_, c)| c.count == 1)
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    match c1.len() {
        0 => {}
        2 => {
            let (index_i, index_j) = (cs[c1[0]].index[0], cs[c1[1]].index[0]);
            ans.push((index_i, index_j));
            assert!(d[index_i] == 1);
            assert!(d[index_j] == 1);
            d[index_i] -= 1;
            d[index_j] -= 1;
            assert!(!dsu.same(index_i, index_j));
            dsu.merge(index_i, index_j);
        }
        _ => {
            println!("-1");
            return;
        }
    }

    // assert
    for u in 0..n {
        if d[u] != 0 {
            println!("-1");
            return;
        }
        if !dsu.same(0, u) {
            println!("-1");
            return;
        }
    }

    for (u, v) in ans {
        println!("{} {}", u + 1, v + 1);
    }
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
