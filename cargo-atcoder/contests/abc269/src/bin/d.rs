use std::collections::HashMap;

use proconio::input;

fn dfs(used: &mut Vec<bool>, map: &HashMap<(i64, i64), usize>, x: i64, y: i64) {
    let next = vec![
        (x - 1, y - 1),
        (x - 1, y),
        (x, y - 1),
        (x, y + 1),
        (x + 1, y),
        (x + 1, y + 1),
    ];
    for (i, j) in next {
        match map.get(&(i, j)) {
            None => continue,
            Some(&index) => {
                if used[index] {
                    continue;
                }
                used[index] = true;
                dfs(used, map, i, j);
            }
        }
    }
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut map = HashMap::new();
    for (i, (x, y)) in xy.iter().copied().enumerate() {
        map.insert((x, y), i);
    }
    let mut count = 0;
    let mut used = vec![false; n];
    for (i, (x, y)) in xy.iter().copied().enumerate() {
        if used[i] {
            continue;
        }
        used[i] = true;
        count += 1;
        dfs(&mut used, &map, x, y);
    }
    let ans = count;
    println!("{}", ans);
}
