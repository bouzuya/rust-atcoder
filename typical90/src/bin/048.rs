use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        k: usize,
        ab: [(usize, usize); n],
    };
    let mut c = vec![];
    for (i, &(a_i, b_i)) in ab.iter().enumerate() {
        c.push((a_i - b_i, 2, a_i, i));
        c.push((b_i, 1, b_i, i));
    }
    c.sort_by_key(|&(v, _, _, _)| cmp::Reverse(v));

    let mut sum = 0_usize;
    let mut time = 0_usize;
    let mut used = vec![false; n];
    for &(_, t, v, i) in c.iter() {
        let t_x = t - if used[i] { 1 } else { 0 };
        if time + t_x > k {
            continue;
        }
        time += t_x;
        sum += v - if used[i] { ab[i].1 } else { 0 };
        used[i] = true;
        if time == k {
            break;
        }
    }

    let ans = sum;
    println!("{}", ans);
}
