use std::collections::HashSet;

use proconio::input;

fn f(m: usize) -> usize {
    m.to_string()
        .chars()
        .map(|c| (c as u8 - b'0') as usize)
        .product::<usize>()
}

fn dfs(set: &mut HashSet<usize>, digits: &mut Vec<usize>, next: usize) {
    if digits.len() == 11 {
        return;
    }

    for d in next..=9 {
        digits.push(d);
        let f_m = f(digits
            .iter()
            .copied()
            .map(|j| (j as u8 + b'0') as char)
            .collect::<String>()
            .parse::<usize>()
            .unwrap());
        set.insert(f_m);
        dfs(set, digits, d);
        digits.pop();
    }
}

fn main() {
    input! {
        n: usize,
        b: usize,
    };
    let mut digits = vec![];
    let mut set = HashSet::new();
    dfs(&mut set, &mut digits, 0);
    let ans = set
        .into_iter()
        .filter(|f_m| {
            let m = b + f_m;
            (1..=n).contains(&m) && m - f(m) == b
        })
        .count();
    println!("{}", ans);
}
