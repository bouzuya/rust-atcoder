// WA
use fenwicktree::*;
use std::{cmp::Reverse, collections::BTreeMap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let mut map = BTreeMap::new();
    for b_i in b.iter().copied() {
        *map.entry(b_i).or_insert(0) += 1_usize;
    }
    let mut count = 0;
    let mut b2 = BTreeMap::new();
    for (i, (k, v)) in map.iter().enumerate() {
        count += v.saturating_sub(1);
        b2.insert(k, i + count);
    }

    let mut ab = a
        .into_iter()
        .zip(b.into_iter())
        .collect::<Vec<(usize, usize)>>();
    ab.sort_by_key(|&(a, b)| (a, Reverse(b)));

    let mut ans = n;
    let mut bit = FenwickTree::new(n, 0);
    for (i, (_, b)) in ab.into_iter().enumerate() {
        bit.add(b2[&b], 1);
        ans += i - bit.accum(b2[&b]);
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
