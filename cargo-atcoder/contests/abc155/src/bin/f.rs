use proconio::input;

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
    use std::cmp::Ordering::*;
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

pub fn upper_bound_by_key<T, K, F>(s: &[T], k: &K, f: F) -> usize
where
    K: std::cmp::Ord,
    F: Fn(&T) -> K,
{
    upper_bound_by(s, |i| f(i).cmp(&k))
}

fn dfs(
    bv: &Vec<bool>,
    cv: &mut Vec<bool>,
    evv: &Vec<Vec<(usize, usize)>>,
    v1: usize,
    ans: &mut Vec<usize>,
) -> bool {
    cv[v1] = true;
    let mut res = bv[v1];
    for &(v2, eno) in evv[v1].iter() {
        if cv[v2] {
            continue;
        }
        let r = dfs(bv, cv, evv, v2, ans);
        if r {
            ans.push(eno);
        }
        res ^= r;
    }
    res
}

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abv: [(usize, usize); n],
        lrv: [(usize, usize); m]
    };
    abv.sort();

    // 隣接要素と一致していない (flip すべき) 箇所
    let mut bv = vec![false; abv.len() + 1];
    bv[0] = bv[0] ^ (abv[0].1 != 0);
    for i in 0..abv.len() - 1 {
        bv[i + 1] = (abv[i].1 != 0) ^ (abv[i + 1].1 != 0);
    }
    bv[abv.len()] = (abv[abv.len() - 1].1 != 0) ^ bv[abv.len()];

    // evv[from] = [(to, edge_no), ...]
    let mut evv: Vec<Vec<(usize, usize)>> = vec![vec![]; bv.len()];
    for i in 0..lrv.len() {
        let (l, r) = lrv[i];
        let li = lower_bound_by_key(&abv, &l, |&(a, _)| a);
        let ri = upper_bound_by_key(&abv, &r, |&(a, _)| a);
        evv[li].push((ri, i + 1));
        evv[ri].push((li, i + 1));
    }

    let mut ans = vec![];
    let mut cv = vec![false; bv.len()];
    for i in 0..bv.len() {
        if cv[i] {
            continue;
        }
        if dfs(&bv, &mut cv, &evv, i, &mut ans) {
            println!("{}", -1);
            return;
        }
    }

    ans.sort();

    println!("{}", ans.len());
    if !ans.is_empty() {
        for i in 0..ans.len() - 1 {
            print!("{} ", ans[i]);
        }
        println!("{}", ans[ans.len() - 1]);
    }
}
