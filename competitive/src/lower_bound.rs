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
