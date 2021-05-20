use std::collections::BTreeMap;

use proconio::{input, marker::Usize1};

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn dfs(e: &Vec<Vec<usize>>, cs: &mut Vec<BTreeMap<usize, usize>>, u: usize, p: usize, pc: usize) {
    let mut c = 1;
    for &v in e[u].iter() {
        if v == p {
            continue;
        }
        if c == pc {
            c += 1;
        }
        cs[if u < v { u } else { v }].insert(if u < v { v } else { u }, c);
        dfs(e, cs, v, u, c);
        c += 1;
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);
    let k = e.iter().map(|e_i| e_i.len()).max().unwrap();
    let mut cs = vec![BTreeMap::new(); n];
    dfs(&e, &mut cs, 0, 0, k + 1);
    println!("{}", k);
    for (a_i, b_i) in ab {
        let c = *cs[a_i].get(&b_i).unwrap();
        println!("{}", c);
    }
}
