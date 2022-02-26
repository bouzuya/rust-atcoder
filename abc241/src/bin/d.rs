use fenwicktree::*;
use std::collections::{BTreeMap, BTreeSet};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut r = vec![];
    for _ in 0..q {
        input! { t: usize }
        r.push(match t {
            1 => {
                input! { x: usize }
                (t, x, 0)
            }
            2 => {
                input! { x: usize, k: usize }
                (t, x, k)
            }
            3 => {
                input! { x: usize, k: usize }
                (t, x, k)
            }
            _ => unreachable!(),
        });
    }

    let map = r
        .iter()
        .copied()
        .map(|(_, x, _)| x)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let mut rev = vec![0; q];
    for (&k, &v) in map.iter() {
        rev[v] = k;
    }
    // println!("{:?}", map);
    // println!("{:?}", rev);

    let mut bit = FenwickTree::new(q + 2, 0);
    for (t, x, k) in r {
        let i = *map.get(&x).unwrap();
        match t {
            1 => {
                bit.add(i, 1);
            }
            2 => {
                let count = bit.sum(0, i + 1);
                if count < k {
                    println!("-1");
                } else {
                    let mut l = 0;
                    let mut r = i + 1;
                    while r - l > 1 {
                        let m = l + (r - l) / 2;
                        let count = bit.sum(m, i + 1);
                        if count >= k {
                            l = m;
                        } else {
                            r = m;
                        }
                    }
                    println!("{}", rev[l]);
                }
            }
            3 => {
                let count = bit.sum(i, q + 1);
                if count < k {
                    println!("-1");
                } else {
                    let mut l = i;
                    let mut r = q + 1;
                    while r - l > 1 {
                        let m = l + (r - l) / 2;
                        let c = bit.sum(i, m);
                        // println!("{} {} {} {}", l, r, m, c);
                        if c >= k {
                            r = m;
                        } else {
                            l = m;
                        }
                    }
                    // println!("{} {} {} {}", l, r, rev[l], rev[r]);
                    println!("{}", rev[l]);
                }
            }
            _ => unreachable!(),
        }
    }
}

//https://github.com/rust-lang-ja/ac-library-rs

pub mod fenwicktree {
    // Reference: https://en.wikipedia.org/wiki/Fenwick_tree
    pub struct FenwickTree<T> {
        n: usize,
        ary: Vec<T>,
        e: T,
    }

    impl<T: Clone + std::ops::AddAssign<T>> FenwickTree<T> {
        pub fn new(n: usize, e: T) -> Self {
            FenwickTree {
                n,
                ary: vec![e.clone(); n],
                e,
            }
        }
        pub fn accum(&self, mut idx: usize) -> T {
            let mut sum = self.e.clone();
            while idx > 0 {
                sum += self.ary[idx - 1].clone();
                idx &= idx - 1;
            }
            sum
        }
        /// performs data[idx] += val;
        pub fn add<U: Clone>(&mut self, mut idx: usize, val: U)
        where
            T: std::ops::AddAssign<U>,
        {
            let n = self.n;
            idx += 1;
            while idx <= n {
                self.ary[idx - 1] += val.clone();
                idx += idx & idx.wrapping_neg();
            }
        }
        /// Returns data[l] + ... + data[r - 1].
        pub fn sum(&self, l: usize, r: usize) -> T
        where
            T: std::ops::Sub<Output = T>,
        {
            self.accum(r) - self.accum(l)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn fenwick_tree_works() {
            let mut bit = FenwickTree::new(5, 0i64);
            // [1, 2, 3, 4, 5]
            for i in 0..5 {
                bit.add(i, i as i64 + 1);
            }
            assert_eq!(bit.sum(0, 5), 15);
            assert_eq!(bit.sum(0, 4), 10);
            assert_eq!(bit.sum(1, 3), 5);
        }
    }
}
