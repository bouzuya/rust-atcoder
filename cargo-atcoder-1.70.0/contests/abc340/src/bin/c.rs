use std::collections::HashMap;

use proconio::input;

fn dfs(memo: &mut HashMap<usize, usize>, x: usize) -> usize {
    if x == 0 || x == 1 {
        return 0;
    }
    if let Some(&v) = memo.get(&x) {
        return v;
    }
    let v = x + dfs(memo, x / 2) + dfs(memo, (x + 1) / 2);
    memo.insert(x, v);
    v
}

fn main() {
    input! {
        n: usize,
    };
    let ans = dfs(&mut HashMap::new(), n);
    println!("{}", ans);
}
