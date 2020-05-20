use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, ab: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(a, b) in ab.iter() {
        e[a].push(b);
        e[b].push(a);
    }
    e
}

fn dfs(e: &Vec<Vec<usize>>, u: usize, p: usize, t: usize) -> Result<usize, Vec<usize>> {
    if u == t {
        return Err(vec![]);
    }
    let mut c = 1;
    for &v in e[u].iter() {
        if v == p {
            continue;
        }
        match dfs(e, v, u, t) {
            Ok(c_v) => c += c_v,
            Err(mut v) => {
                v.push(u);
                return Err(v);
            }
        }
    }
    Ok(c)
}

fn dfs2(e: &Vec<Vec<usize>>, u: usize, p: usize, v_b: usize, v_w: usize) -> usize {
    let mut c = 1;
    for &v in e[u].iter() {
        if v == p || v == v_b || v == v_w {
            continue;
        }
        c += dfs2(e, v, u, v_b, v_w);
    }
    c
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
    };

    let e = adjacency_list(n, &ab);

    let mut us = vec![];
    let mut c_b = 1;
    for &v in e[0].iter() {
        match dfs(&e, v, 0, n - 1) {
            Ok(c) => c_b += c,
            Err(s) => us = s,
        }
    }
    us.push(n - 1);
    if us.len() > 2 {
        let mut v_b = 0;
        for w in us[1..(us.len() - 2) / 2].windows(2) {
            match w {
                &[u, v_w] => {
                    c_b += dfs2(&e, u, u, v_b, v_w);
                    v_b = u;
                }
                _ => unreachable!(),
            }
        }
    }

    let c_w = n - c_b;
    let ans = if c_b > c_w { "Fennec" } else { "Snuke" };
    println!("{}", ans);
}
