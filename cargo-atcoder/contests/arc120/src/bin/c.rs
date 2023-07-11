use std::collections::BTreeMap;

use fenwicktree::*;
use proconio::input;

fn inversion_number(a: &[usize]) -> usize {
    let mut count = 0;
    let mut ft = FenwickTree::new(a.len(), 0);
    for (i, &a_i) in a.iter().enumerate() {
        ft.add(a_i, 1);
        count += i - ft.accum(a_i);
    }
    count
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let a = a
        .into_iter()
        .enumerate()
        .map(|(i, a_i)| a_i + i)
        .collect::<Vec<usize>>();
    let b = b
        .into_iter()
        .enumerate()
        .map(|(i, b_i)| b_i + i)
        .collect::<Vec<usize>>();

    let mut map_a = BTreeMap::new();
    for (i, a_i) in a.into_iter().enumerate() {
        map_a.entry(a_i).or_insert_with(|| vec![]).push(i);
    }
    let mut map_b = BTreeMap::new();
    for (j, b_j) in b.into_iter().enumerate() {
        map_b.entry(b_j).or_insert_with(|| vec![]).push(j);
    }

    let mut s = vec![0; n];
    for (a_i, is) in map_a.into_iter() {
        match map_b.get_mut(&a_i) {
            None => {
                println!("-1");
                return;
            }
            Some(js) => {
                if is.len() != js.len() {
                    println!("-1");
                    return;
                }
                for (&i, &j) in is.iter().zip(js.iter()) {
                    s[i] = j;
                }
            }
        }
    }

    let ans = inversion_number(&s);
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
