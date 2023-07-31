use std::collections::HashSet;

use proconio::input;

fn dfs(set: &mut HashSet<String>, v: &mut Vec<char>, n: usize) {
    if v.len() > n {
        return;
    }
    let s = v.iter().collect::<String>();
    if s.contains('3') && s.contains('5') && s.contains('7') {
        set.insert(s);
    }
    for &c in &['3', '5', '7'] {
        v.push(c);
        dfs(set, v, n);
        v.pop();
    }
}

fn main() {
    input! {
        n: usize,
    };
    let mut set = HashSet::new();
    dfs(&mut set, &mut vec![], n.to_string().len());
    let ans = set
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .filter(|x| x <= &n)
        .count();
    println!("{}", ans);
}
