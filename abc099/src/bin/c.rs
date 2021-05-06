use std::cmp;

use proconio::input;

fn dfs(min_count: &mut usize, n: usize, cs: &Vec<usize>, i: usize, count: usize, sum: usize) {
    if sum > n {
        return;
    }
    if i == cs.len() {
        *min_count = cmp::min(*min_count, count + n - sum);
        return;
    }
    for j in 0..=if cs[i] % 6 == 0 { 6 } else { 9 } {
        if sum + cs[i] * j > n {
            break;
        }
        dfs(min_count, n, cs, i + 1, count + j, sum + cs[i] * j);
    }
}

fn main() {
    input! {
        n: usize,
    };
    let max_n = 100_000_usize;
    let mut cs = vec![];
    let mut i = 6;
    while i <= max_n {
        cs.push(i);
        i *= 6;
    }
    let mut i = 9;
    while i <= max_n {
        cs.push(i);
        i *= 9;
    }
    cs.sort();
    cs.reverse();

    let mut min_count = max_n + 1;
    dfs(&mut min_count, n, &cs, 0, 0, 0);
    let ans = min_count;
    println!("{}", ans);
}
