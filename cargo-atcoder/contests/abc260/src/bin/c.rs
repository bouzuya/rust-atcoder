use std::collections::HashMap;

use proconio::input;

fn dfs(
    x: usize,
    y: usize,
    memo: &mut HashMap<(char, usize), usize>,
    color: char,
    level: usize,
    count: usize,
) -> usize {
    if let Some(v) = memo.get(&(color, level)) {
        return v * count;
    }

    let res = match (color, level) {
        ('R', 1) => 0,
        ('R', n) => dfs(x, y, memo, 'R', n - 1, 1) + dfs(x, y, memo, 'B', n, 1) * x,
        ('B', 1) => 1,
        ('B', n) => dfs(x, y, memo, 'R', n - 1, 1) + dfs(x, y, memo, 'B', n - 1, 1) * y,
        (_, _) => unreachable!(),
    };
    memo.insert((color, level), res);
    res * count
}

fn main() {
    input! {
        n: usize,
        x: usize,
        y: usize,
    };
    let mut memo = HashMap::new();
    let ans = dfs(x, y, &mut memo, 'R', n, 1);
    println!("{}", ans);
}
