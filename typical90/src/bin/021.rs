use proconio::input;
use proconio::marker::Usize1;

fn dfs(e: &Vec<Vec<usize>>, u: usize, used: &mut Vec<bool>, vs: &mut Vec<usize>) {
    used[u] = true;
    for &v in e[u].iter() {
        if used[v] {
            continue;
        }
        dfs(e, v, used, vs);
    }
    vs.push(u);
}

fn rdfs(e: &Vec<Vec<usize>>, u: usize, used: &mut Vec<bool>, res: &mut Vec<usize>) {
    used[u] = true;
    res.push(u);
    for &v in e[u].iter() {
        if used[v] {
            continue;
        }
        rdfs(e, v, used, res);
    }
}

// 強連結成分分解 (SCC)
fn strongly_connected_component(v_len: usize, es: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    // 帰りがけ順 (post order) での頂点を得る
    let vs = {
        let e = {
            let mut e = vec![vec![]; v_len];
            for &(u, v) in es.iter() {
                e[u].push(v);
            }
            e
        };
        let mut vs: Vec<usize> = vec![];
        let mut used: Vec<bool> = vec![false; v_len];
        for u in 0..v_len {
            if used[u] {
                continue;
            }
            dfs(&e, u, &mut used, &mut vs);
        }
        vs
    };

    let re = {
        let mut e = vec![vec![]; v_len];
        for &(u, v) in es.iter() {
            e[v].push(u);
        }
        e
    };
    let mut res = vec![];
    let mut used: Vec<bool> = vec![false; v_len];
    for &u in vs.iter().rev() {
        if used[u] {
            continue;
        }
        let mut vs = vec![];
        rdfs(&re, u, &mut used, &mut vs);
        res.push(vs);
    }
    res
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let scc = strongly_connected_component(n, &ab);
    let mut ans = 0;
    for g in scc {
        if g.len() == 1 {
            continue;
        }
        ans += g.len() * (g.len() - 1) / 2;
    }
    println!("{}", ans);
}
