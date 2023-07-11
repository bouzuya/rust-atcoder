use std::{cmp, collections::VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [i64; n],
    };
    a.sort();
    let mut a = a.into_iter().collect::<VecDeque<i64>>();
    let a0 = a.pop_front().unwrap();
    let an = a.pop_back().unwrap();
    let mut sum = an - a0;
    let mut cur = (a0, an);
    while !a.is_empty() {
        let front = *a.get(0).unwrap();
        let back = *a.get(a.len() - 1).unwrap();
        let (c_f, c_b) = cur;
        let f = |x: i64| cmp::max((c_f - x).abs(), (c_b - x).abs());
        let v = if f(front) > f(back) {
            a.pop_front().unwrap()
        } else {
            a.pop_back().unwrap()
        };
        let (v_f, v_b) = ((c_f - v).abs(), (c_b - v).abs());
        cur = if v_f > v_b {
            sum += v_f;
            (v, c_b)
        } else {
            sum += v_b;
            (c_f, v)
        };
    }
    let ans = sum;
    println!("{}", ans);
}
