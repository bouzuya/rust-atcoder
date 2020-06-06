use proconio::input;
use superslice::*;

fn main() {
    input! {
        n: usize,
        mut wh: [(i64, i64); n],
    };
    wh.sort_by_key(|&(w, h)| (w, -h));

    // LIS: Longest Increasing Subsequence の長さを求める
    let mut l = vec![wh[0].1];
    for &(_, h) in wh.iter().skip(1) {
        if l[l.len() - 1] < h {
            l.push(h);
        } else {
            let i = l.lower_bound(&h);
            l[i] = h;
        }
    }

    let ans = l.len();
    println!("{}", ans);
}
