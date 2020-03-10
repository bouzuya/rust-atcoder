use self::lower_bound::*;
use self::segment_tree::SegmentTree;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut xdv: [(i64, i64); n],
    };
    xdv.sort();

    let mut st = SegmentTree::new(n);
    for i in 0..n {
        st.update(i, i);
    }
    for l in (0..n).rev() {
        let l_xd = xdv[l].0 + xdv[l].1;
        let r = lower_bound_by(&xdv, |&(x, _)| x.cmp(&l_xd));
        let r_i = st.query(l..r);
        st.update(l, r_i);
    }

    let mod_p = 998244353;
    let mut dp = vec![0usize; n + 1];
    dp[n] = 1;
    for i in (0..n).rev() {
        let j = st.query(i..i + 1);
        dp[i] = (dp[i + 1] + dp[j + 1]) % mod_p;
    }

    let ans = dp[0];
    println!("{}", ans);
}

mod lower_bound {
    pub fn lower_bound<T>(s: &[T], x: &T) -> usize
    where
        T: std::cmp::Ord,
    {
        lower_bound_by(s, |i| i.cmp(x))
    }

    pub fn lower_bound_by<T, F>(s: &[T], f: F) -> usize
    where
        F: Fn(&T) -> std::cmp::Ordering,
    {
        use std::cmp::Ordering::Less;
        let mut b = 0;
        let mut l = s.len();
        while l > 1 {
            let h = l / 2;
            let m = b + h;
            b = if f(&s[m]) != Less { b } else { m };
            l -= h;
        }
        b + if f(&s[b]) != Less { 0 } else { 1 }
    }

    pub fn lower_bound_by_key<T, K, F>(s: &[T], k: &K, f: F) -> usize
    where
        K: std::cmp::Ord,
        F: Fn(&T) -> K,
    {
        lower_bound_by(s, |i| f(i).cmp(&k))
    }
}

mod segment_tree {
    pub struct SegmentTree {
        dv: Vec<usize>,
    }

    impl SegmentTree {
        pub fn new(n: usize) -> Self {
            let dv = vec![0usize; n.next_power_of_two() * 2];
            Self { dv }
        }

        pub fn query(&self, range: std::ops::Range<usize>) -> usize {
            self.q(&range, 0, 0..(self.dv.len() / 2))
        }

        pub fn update(&mut self, i: usize, v: usize) {
            let mut j = i + (self.dv.len() / 2) - 1;
            self.dv[j] = v;
            while j > 0 {
                j = (j - 1) >> 1;
                let l = self.dv[j * 2 + 1];
                let r = self.dv[j * 2 + 2];

                let v = std::cmp::max(l, r);

                self.dv[j] = v;
            }
        }

        fn q(&self, rq: &std::ops::Range<usize>, i: usize, ri: std::ops::Range<usize>) -> usize {
            if rq.end <= ri.start || ri.end <= rq.start {
                0
            } else if rq.start <= ri.start && ri.end <= rq.end {
                self.dv[i]
            } else {
                let m = ri.start + (ri.end - ri.start) / 2;
                let l = self.q(&rq, i * 2 + 1, ri.start..m);
                let r = self.q(&rq, i * 2 + 2, m..ri.end);

                std::cmp::max(l, r)
            }
        }
    }
}
