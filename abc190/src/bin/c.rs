use std::cmp::max;

use proconio::input;
use proconio::marker::Usize1;

fn dfs(s: &mut Vec<bool>, i: usize, ab: &Vec<(usize, usize)>, cd: &Vec<(usize, usize)>) -> usize {
    if i == cd.len() {
        let mut count = 0;
        for &(a_i, b_i) in ab.iter() {
            if s[a_i] && s[b_i] {
                count += 1;
            }
        }
        return count;
    }

    let mut max_v = 0;
    if s[cd[i].0] {
        max_v = max(max_v, dfs(s, i + 1, ab, cd));
    } else {
        s[cd[i].0] = true;
        max_v = max(max_v, dfs(s, i + 1, ab, cd));
        s[cd[i].0] = false;
    }
    if s[cd[i].1] {
        max_v = max(max_v, dfs(s, i + 1, ab, cd));
    } else {
        s[cd[i].1] = true;
        max_v = max(max_v, dfs(s, i + 1, ab, cd));
        s[cd[i].1] = false;
    }
    max_v
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
        k: usize,
        cd: [(Usize1, Usize1); k],
    };
    let mut s = vec![false; n];
    let ans = dfs(&mut s, 0, &ab, &cd);
    println!("{}", ans);
}
