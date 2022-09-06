use fenwicktree::*;
use std::collections::{BTreeSet, HashMap};

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut query = vec![];
    for _ in 0..q {
        input! {
            t: usize,
        }
        match t {
            1 => {
                input! {
                    x: usize,
                }
                query.push((1, x, 0));
            }
            2 => {
                input! {
                    x: usize,
                    k: usize,
                }
                query.push((2, x, k));
            }
            3 => {
                input! {
                    x: usize,
                    k: usize,
                }
                query.push((3, x, k));
            }
            _ => unreachable!(),
        }
    }

    let mx = query
        .iter()
        .copied()
        .map(|(_, x, _)| x)
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(HashMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let mut rx = HashMap::new();
    for (k, v) in mx.iter() {
        rx.insert(*v, *k);
    }

    let n = mx.len();
    let mut count = FenwickTree::new(n + 1, 0);
    for (t, x, k) in query {
        let y = *mx.get(&x).unwrap();
        match t {
            1 => {
                count.add(y, 1);
            }
            2 => match count.sum(0, y + 1).cmp(&k) {
                std::cmp::Ordering::Less => {
                    println!("-1");
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    let mut l = 0;
                    let mut r = y + 1;
                    while r - l > 1 {
                        let mid = l + (r - l) / 2;
                        match count.sum(mid, y + 1).cmp(&k) {
                            std::cmp::Ordering::Less => {
                                r = mid;
                            }
                            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                                l = mid;
                            }
                        }
                    }
                    let ans = rx.get(&l).unwrap();
                    println!("{}", ans);
                }
            },
            3 => match count.sum(y, n + 1).cmp(&k) {
                std::cmp::Ordering::Less => {
                    println!("-1");
                }
                std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                    let mut l = y;
                    let mut r = n + 1;
                    while r - l > 1 {
                        let mid = l + (r - l) / 2;
                        match count.sum(y, mid).cmp(&k) {
                            std::cmp::Ordering::Less => {
                                l = mid;
                            }
                            std::cmp::Ordering::Equal | std::cmp::Ordering::Greater => {
                                r = mid;
                            }
                        }
                    }
                    let ans = rx.get(&l).unwrap();
                    println!("{}", ans);
                }
            },
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
