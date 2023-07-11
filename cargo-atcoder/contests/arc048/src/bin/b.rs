use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        rh: [(i64, i64); n],
    };
    let mut irh = rh
        .iter()
        .enumerate()
        .map(|(i, &(r_i, h_i))| (i, r_i, h_i))
        .collect::<Vec<(usize, i64, i64)>>();
    irh.sort_by_key(|&(i_i, r_i, h_i)| (r_i, h_i, i_i));
    let mut ans = vec![(0, 0, 0); n];
    for i in 0..n {
        let (i_i, r_i, h_i) = irh[i];
        let c1 = irh.upper_bound_by_key(&(r_i - 1), |(_, r_j, _)| *r_j);
        let l = irh.lower_bound_by_key(&r_i, |(_, r_j, _)| *r_j);
        let r = irh.lower_bound_by_key(&(r_i + 1), |(_, r_j, _)| *r_j);
        if r == l {
            ans[i_i] = (c1, n - c1, 0);
        } else {
            let h = &irh[l..r];
            let rock = h.upper_bound_by_key(&1, |(_, _, h_j)| *h_j);
            let k = h.upper_bound_by_key(&2, |(_, _, h_j)| *h_j);
            let scissors = k - rock;
            let paper = (r - l) - (rock + scissors);
            // println!("r = {} s = {} p = {}", rock, scissors, paper);
            match h_i {
                1 => {
                    let w = c1 + scissors;
                    let t = rock;
                    let l = n - (w + t);
                    ans[i_i] = (w, l, t - 1);
                }
                2 => {
                    let w = c1 + paper;
                    let t = scissors;
                    let l = n - (w + t);
                    ans[i_i] = (w, l, t - 1);
                }
                3 => {
                    let w = c1 + rock;
                    let t = paper;
                    let l = n - (w + t);
                    ans[i_i] = (w, l, t - 1);
                }
                _ => unreachable!("unknown hand"),
            }
        }
    }
    for i in 0..n {
        let (w, l, t) = ans[i];
        println!("{} {} {}", w, l, t);
    }
}
