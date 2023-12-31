use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn dfs(set: &mut HashSet<Vec<char>>, t: &mut Vec<char>) {
    if t.len() == 3 {
        return;
    }
    for c in (0..26)
        .map(|j| (b'a' + j) as char)
        .chain(std::iter::once('.'))
    {
        t.push(c);
        set.insert(t.clone());
        dfs(set, t);
        t.pop();
    }
}

fn main() {
    input! {
        s: Chars,
    };
    let mut set = HashSet::new();
    let mut t = vec![];
    dfs(&mut set, &mut t);

    let mut count = 0_usize;
    for t in set {
        if s.len() < t.len() {
            continue;
        }
        for i in 0..s.len() + 1 - t.len() {
            if t.iter()
                .copied()
                .enumerate()
                .all(|(j, c)| c == '.' || s[i + j] == c)
            {
                count += 1;
                break;
            }
        }
    }
    let ans = count;
    println!("{}", ans);
}
