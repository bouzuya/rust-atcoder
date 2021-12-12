use fenwicktree::*;
use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet},
};

use proconio::input;

fn compress(a: &[usize]) -> BTreeMap<usize, usize> {
    a.iter()
        .copied()
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        })
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };

    let b2 = compress(&b);
    let mut ab = a
        .into_iter()
        .zip(b.into_iter())
        .collect::<Vec<(usize, usize)>>();
    ab.sort_by_key(|&(a, b)| (Reverse(a), b));

    let mut ans = 0_usize;
    let mut bit = FenwickTree::new(n, 0_usize);
    let mut i = 0_usize;
    while i < n {
        let (a, b) = ab[i];
        let mut j = i + 1;
        while j < n && ab[j] == (a, b) {
            j += 1;
        }
        let count = j - i;
        ans += count * (count + bit.accum(b2[&b] + 1));
        bit.add(b2[&b], count);
        i = j;
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
