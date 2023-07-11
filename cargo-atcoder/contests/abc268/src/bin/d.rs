use std::collections::HashSet;

use proconio::input;
use superslice::Ext;

fn dfs(
    underscores: &mut HashSet<Vec<usize>>,
    cur: &mut Vec<usize>,
    n: usize,
    muc: usize,
    c: usize,
) {
    if c == muc {
        return;
    }
    for i in 0..n - 1 {
        cur[i] += 1;
        underscores.insert(cur.clone());
        dfs(underscores, cur, n, muc, c + 1);
        cur[i] -= 1;
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String; n],
        t: [String; m],
    };

    let ng = t.into_iter().collect::<HashSet<_>>();
    let min_len = s.iter().map(|s_i| s_i.len()).sum::<usize>() + n - 1;

    let muc = 16 - min_len;
    let mut underscores = HashSet::new();
    let mut cur = vec![0; n];
    dfs(&mut underscores, &mut cur, n, muc, 0);

    let mut is = (0..n).collect::<Vec<usize>>();
    loop {
        let mut x = vec![];
        for i in 0..n {
            x.push(format!("{}", s[is[i]]));
        }
        let x = x.join("_");
        if !ng.contains(&x) && x.len() >= 3 {
            println!("{}", x);
            return;
        }
        for u in underscores.iter() {
            let mut x = vec![];
            for i in 0..n {
                x.push(format!("{}{}", s[is[i]], "_".repeat(u[i])));
            }
            let x = x.join("_");
            if !ng.contains(&x) && x.len() >= 3 {
                println!("{}", x);
                return;
            }
        }
        if !is.next_permutation() {
            break;
        }
    }
    println!("-1");
}
