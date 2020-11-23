use dsu::*;
use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        c: [Usize1; n],
    };
    let mut maps = vec![];
    for c_i in c {
        let mut map = std::collections::BTreeMap::new();
        map.insert(c_i, 1);
        maps.push(std::rc::Rc::new(std::cell::RefCell::new(map)));
    }
    let mut dsu = Dsu::new(n);
    for _ in 0..q {
        input! { t: usize };
        match t {
            1 => {
                input! {
                    a: Usize1,
                    b: Usize1,
                };
                let l_a = dsu.leader(a);
                let l_b = dsu.leader(b);
                if l_a != l_b {
                    let m_a = std::rc::Rc::clone(&maps[l_a]);
                    let m_b = std::rc::Rc::clone(&maps[l_b]);
                    if dsu.size(a) < dsu.size(b) {
                        let m_a = m_a.borrow();
                        let mut m_b = m_b.borrow_mut();
                        for (k, v) in m_a.iter() {
                            *m_b.entry(k.clone()).or_insert(0) += v.clone();
                        }
                    } else {
                        let mut m_a = m_a.borrow_mut();
                        let m_b = m_b.borrow();
                        for (k, v) in m_b.iter() {
                            *m_a.entry(k.clone()).or_insert(0) += v.clone();
                        }
                    }
                }
                dsu.merge(a, b);
            }
            2 => {
                input! {
                    x: Usize1,
                    y: Usize1,
                };
                let l = dsu.leader(x);
                let m_x = maps[l].clone();
                let map = m_x.borrow();
                let ans = map.get(&y).unwrap_or(&0);
                println!("{}", ans);
            }
            _ => unreachable!(),
        }
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
