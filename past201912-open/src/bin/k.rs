use proconio::input;
use proconio::marker::{Isize1, Usize1};

fn dfs(level: &mut Vec<usize>, e: &Vec<Vec<usize>>, u: usize, p: usize, l: usize) {
    level[u] = l;
    for &v in e[u].iter() {
        if v == p {
            continue;
        }
        dfs(level, e, v, u, l + 1)
    }
}

fn lca(tbl: &Vec<Vec<usize>>, l: &Vec<usize>, u: usize, p: usize) -> usize {
    // 問題の制約と事前検査より u != p && l[u] > l[p]
    let d_l = l[u] - l[p];
    let tbl_h = tbl.len();
    let mut c = u;
    for i in 0..tbl_h {
        if ((d_l >> i) & 1) == 1 {
            c = tbl[i][c];
        }
    }
    // 深さを揃えて一致するかを確認する
    c
    // TODO: この問題では不要であるため p 側の祖先の探索はしない
}

fn main() {
    input! {
        n: usize,
        p: [Isize1; n],
        q: usize,
        ab: [(Usize1, Usize1); q],
    };
    // a, b の LCA: Lowest Common Ancestor (最小共通祖先) が b と一致すれば Yes
    // ダブリングの前処理
    let tbl_h = n.next_power_of_two().trailing_zeros() as usize + 1;
    let mut tbl = vec![vec![n; n]; tbl_h];
    for (i, &p_i) in p.iter().enumerate() {
        tbl[0][i] = if p_i < 0 { i } else { p_i as usize };
    }
    for i in 1..tbl_h {
        for j in 0..n {
            tbl[i][j] = tbl[i - 1][tbl[i - 1][j]];
        }
    }

    // 深さの計算
    let mut root = n;
    let mut e = vec![vec![]; n + 1];
    for (i, &p_i) in p.iter().enumerate() {
        if p_i < 0 {
            root = i;
        } else {
            e[p_i as usize].push(i);
        }
    }
    let mut l = vec![n; n];
    dfs(&mut l, &e, root, root, 0);

    for &(a, b) in ab.iter() {
        let ans = if l[a] <= l[b] {
            "No"
        } else {
            if lca(&tbl, &l, a, b) == b {
                "Yes"
            } else {
                "No"
            }
        };
        println!("{}", ans);
    }
}
