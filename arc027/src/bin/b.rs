use dsu::*;
use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        _: usize,
        s_1: Chars,
        s_2: Chars,
    };
    let mut is_used = vec![false; 26];
    let mut is_digit = vec![false; 26];
    let mut dsu = Dsu::new(26);
    for (&c_1, &c_2) in s_1.iter().zip(s_2.iter()) {
        match (c_1.is_digit(10), c_2.is_digit(10)) {
            (true, true) => {}
            (true, false) => {
                let i_2 = (c_2 as u8 - 'A' as u8) as usize;
                is_digit[i_2] = true;
                is_used[i_2] = true;
            }
            (false, true) => {
                let i_1 = (c_1 as u8 - 'A' as u8) as usize;
                is_digit[i_1] = true;
                is_used[i_1] = true;
            }
            (false, false) => {
                let i_1 = (c_1 as u8 - 'A' as u8) as usize;
                let i_2 = (c_2 as u8 - 'A' as u8) as usize;
                dsu.merge(i_1, i_2);
                is_used[i_1] = true;
                is_used[i_2] = true;
            }
        }
    }
    let mut is_first = vec![false; 26];
    if !s_1[0].is_digit(10) {
        is_first[(s_1[0] as u8 - 'A' as u8) as usize] = true;
    }
    if !s_2[0].is_digit(10) {
        is_first[(s_2[0] as u8 - 'A' as u8) as usize] = true;
    }

    let mut ans = 1_i64;
    for group in dsu.groups() {
        if group.len() == 1 && !is_used[group[0]] {
            continue;
        }
        let d = group.iter().any(|&i| is_digit[i]);
        let f = group.iter().any(|&i| is_first[i]);
        let x = if d {
            1
        } else if f {
            9
        } else {
            10
        };
        ans *= x;
    }
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
