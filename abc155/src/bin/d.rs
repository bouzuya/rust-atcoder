use proconio::input;

fn upper_bound_by_key<F>(v: &[i64], x: &i64, f: F) -> usize
where
    F: Fn(i64) -> i64,
{
    use std::cmp::Ordering;

    let mut l = 0;
    let mut h = v.len();
    while l < h {
        let m = l + (h - l) / 2;
        match f(v[m]).cmp(x) {
            Ordering::Less | Ordering::Equal => {
                l = m + 1;
            }
            Ordering::Greater => {
                h = m;
            }
        }
    }
    l
}

fn count_lte(av: &Vec<i64>, rv: &Vec<i64>, x: i64) -> usize {
    let mut c = 0;
    for i in 0..av.len() {
        let a = av[i];
        if a < 0 {
            c += upper_bound_by_key(&rv[..av.len() - (i + 1)], &x, |b| a * b);
        } else if a == 0 {
            c += if x >= 0 { av.len() - (i + 1) } else { 0 };
        } else {
            c += upper_bound_by_key(&av[i + 1..], &x, |b| a * b);
        }
    }
    c
}

fn main() {
    input! {
        n: usize,
        k: usize,
        mut av: [i64; n]
    };
    av.sort();
    let mut rv = av.clone();
    rv.reverse();

    let mut l = -10i64.pow(18) - 1;
    let mut r = 10i64.pow(18);
    while l + 1 < r {
        let x = (l + r) / 2;
        if count_lte(&av, &rv, x) < k {
            l = x;
        } else {
            r = x;
        }
    }
    let ans = r;
    println!("{}", ans);
}
