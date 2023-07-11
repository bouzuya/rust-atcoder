use dsu::*;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        txyv: [(usize, Usize1, Usize1, i64); q],
    };

    let inf = 2_000_000_001;
    let mut v = vec![inf; n];
    let mut ambiguous = vec![false; q];
    let mut dsu = Dsu::new(n);
    for (i, (t_i, x_i, y_i, v_i)) in txyv.iter().copied().enumerate() {
        match t_i {
            0 => {
                dsu.merge(x_i, y_i);
                v[x_i] = v_i;
            }
            1 => {
                if !dsu.same(x_i, y_i) {
                    ambiguous[i] = true;
                }
            }
            _ => unreachable!(),
        }
    }

    let mut s = vec![(n, inf); n];
    for mut group in dsu.groups() {
        if group.len() == 1 {
            continue;
        }
        group.sort();
        let mut cur = 0;
        let i = group[0];
        s[i] = (i, cur);
        for j in group.into_iter().skip(1) {
            // +, -, +, -, ...
            let d = v[j - 1] - cur;
            s[j] = (i, d);
            cur = d;
        }
    }

    for (i, (t_i, x_i, y_i, v_i)) in txyv.iter().copied().enumerate() {
        if t_i != 1 {
            continue;
        }
        if ambiguous[i] {
            println!("Ambiguous");
            continue;
        }
        let i_start = s[x_i].0;
        let p_x = (x_i - i_start) % 2 == 0;
        let diff = if p_x { 1 } else { -1 } * (v_i - s[x_i].1);
        let p_y = (y_i - i_start) % 2 == 0;
        println!("{}", s[y_i].1 + if p_y { 1 } else { -1 } * diff);
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
