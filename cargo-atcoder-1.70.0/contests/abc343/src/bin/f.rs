use std::collections::{BTreeSet, HashMap};

use ac_library::{Additive, Monoid, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        mut a: [usize; n],
        query: [(usize, usize, usize); q],
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
    struct T(usize, usize);
    impl Monoid for T {
        type S = T;

        fn identity() -> Self::S {
            T(0, 0)
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            let mut set = BTreeSet::new();
            set.insert(a.0);
            set.insert(a.1);
            set.insert(b.0);
            set.insert(b.1);
            let mut iter = set.into_iter().rev();
            T(iter.next().unwrap(), iter.next().unwrap_or(0))
        }
    }

    enum MySet {
        Small(BTreeSet<usize>, usize),
        Large(Segtree<Additive<usize>>, usize),
    }

    impl MySet {
        fn new(n: usize) -> Self {
            MySet::Small(BTreeSet::new(), n)
        }

        fn insert(&mut self, x: usize) {
            match self {
                MySet::Small(set, n) => {
                    set.insert(x);
                    if set.len() > 1000 {
                        let mut st = Segtree::new(*n);
                        for p in set.iter().copied() {
                            st.set(p, 1);
                        }
                        *self = MySet::Large(st, *n);
                    }
                }
                MySet::Large(st, _) => {
                    st.set(x, st.get(x) + 1);
                }
            }
        }

        fn remove(&mut self, x: usize) {
            match self {
                MySet::Small(set, _) => {
                    set.remove(&x);
                }
                MySet::Large(st, _) => {
                    st.set(x, st.get(x) - 1);
                }
            }
        }

        fn count(&self, l: usize, r: usize) -> usize {
            match self {
                MySet::Small(set, _) => set.range(l..r).count(),
                MySet::Large(st, _) => st.prod(l..r),
            }
        }
    }

    let mut max2 = Segtree::<T>::new(n);
    for (i, a_i) in a.iter().copied().enumerate() {
        max2.set(i, T(a_i, 0));
    }
    let mut count = HashMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        let st = count.entry(a_i).or_insert_with(|| MySet::new(n));
        st.insert(i);
    }
    for (t, q1, q2) in query {
        match t {
            1 => {
                let (p, x) = (q1 - 1, q2);
                max2.set(p, T(x, 0));
                let prev = a[p];
                a[p] = x;
                let st = count.get_mut(&prev).unwrap();
                st.remove(p);
                let st = count.entry(x).or_insert_with(|| MySet::new(n));
                st.insert(p);
            }
            2 => {
                let (l, r) = (q1 - 1, q2);
                let T(_, max2) = max2.prod(l..r);
                if max2 == 0 {
                    println!("0");
                } else {
                    let ans = count.get(&max2).unwrap().count(l, r);
                    println!("{}", ans);
                }
            }
            _ => unreachable!(),
        }
    }
}
