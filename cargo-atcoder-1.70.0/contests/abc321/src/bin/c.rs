use std::collections::BTreeSet;

use proconio::input;

fn dfs(set: &mut BTreeSet<usize>, cur: &mut Vec<usize>) {
    let mut x = 0;
    for i in cur.iter().copied() {
        x *= 10;
        x += i;
    }
    set.insert(x);
    let max = cur[cur.len() - 1];
    for i in (0..max).rev() {
        cur.push(i);
        dfs(set, cur);
        cur.pop();
    }
}

fn main() {
    input! {
        k: usize,
    };
    if k <= 9 {
        println!("{k}");
        return;
    }

    let mut set = BTreeSet::new();
    for i in 1..=9 {
        dfs(&mut set, &mut vec![i]);
    }

    let ans = set.iter().nth(k - 1).unwrap();
    println!("{}", ans);
}
