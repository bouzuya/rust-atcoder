use std::collections::{BTreeMap, BTreeSet};

use fenwicktree::*;
use proconio::{input, marker::Usize1};

fn f(a: &[usize]) -> usize {
    let mut count = 0;
    let c = a
        .iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let mut ft = FenwickTree::new(a.len(), 0);
    for (i, a_i) in a.iter().enumerate() {
        count += i - ft.sum(0, c[a_i] + 1);
        ft.add(c[a_i], 1);
    }
    count
}

fn main() {
    input! {
        n: usize,
        c: [Usize1; n],
        x: [Usize1; n],
    };
    let mut d = vec![vec![]; n];
    for (i, c_i) in c.iter().copied().enumerate() {
        d[c_i].push(x[i]);
    }
    let mut ans = f(&x);
    for d_i in d {
        if d_i.is_empty() {
            continue;
        }
        ans -= f(&d_i);
    }
    println!("{}", ans);
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
