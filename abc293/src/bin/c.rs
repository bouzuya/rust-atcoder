use std::collections::HashSet;

use proconio::input;

fn dfs(
    count: &mut usize,
    h: usize,
    w: usize,
    a: &[Vec<usize>],
    set: &mut HashSet<usize>,
    i: usize,
    j: usize,
) {
    if i == h - 1 && j == w - 1 {
        *count += 1;
        return;
    }

    if j + 1 < w && set.insert(a[i][j + 1]) {
        dfs(count, h, w, a, set, i, j + 1);
        set.remove(&a[i][j + 1]);
    }
    if i + 1 < h && set.insert(a[i + 1][j]) {
        dfs(count, h, w, a, set, i + 1, j);
        set.remove(&a[i + 1][j]);
    }
}

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [[usize; w]; h],
    };
    if h == 1 && w == 1 {
        println!("1");
        return;
    }
    let mut count = 0_usize;
    let mut set = HashSet::new();
    set.insert(a[0][0]);
    dfs(&mut count, h, w, &a, &mut set, 0, 0);
    let ans = count;
    println!("{}", ans);
}
