use self::lower_bound::lower_bound;
use self::upper_bound::upper_bound;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut av: [usize; n],
        mut bv: [usize; n],
        mut cv: [usize; n],
    };
    av.sort();
    bv.sort();
    cv.sort();
    let ans: usize = bv
        .iter()
        .map(|&b| lower_bound(&av, &b) * (n - upper_bound(&cv, &b)))
        .sum();
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
}

mod upper_bound {
    pub fn upper_bound<T>(s: &[T], x: &T) -> usize
    where
        T: std::cmp::Ord,
    {
        upper_bound_by(s, |i| i.cmp(x))
    }

    pub fn upper_bound_by<T, F>(s: &[T], f: F) -> usize
    where
        F: Fn(&T) -> std::cmp::Ordering,
    {
        use std::cmp::Ordering::Greater;
        let mut b = 0;
        let mut l = s.len();
        while l > 1 {
            let h = l / 2;
            let m = b + h;
            b = if f(&s[m]) == Greater { b } else { m };
            l -= h;
        }
        b + if f(&s[b]) == Greater { 0 } else { 1 }
    }
}
