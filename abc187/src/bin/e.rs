use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Usize1;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<(usize, usize)>> {
    let mut e = vec![vec![]; n];
    for (i, &(u, v)) in uv.iter().enumerate() {
        e[u].push((v, i));
        e[v].push((u, i));
    }
    e
}

fn dfs(c: &mut Vec<i64>, d: &Vec<i64>, e: &Vec<Vec<(usize, usize)>>, u: usize, u_p: usize, x: i64) {
    c[u] += x;
    for &(v, v_e_i) in e[u].iter() {
        if v == u_p {
            continue;
        }
        dfs(c, d, e, v, u, x + d[v_e_i]);
    }
}

fn main() {
    input! {
        n: usize,
        ab: [(Usize1, Usize1); n - 1],
        q: usize,
        tex: [(Usize1, Usize1, i64); q],
    };

    let e = adjacency_list(n, &ab);

    let p = {
        // bfs
        let mut p = vec![0; n];
        let mut q = VecDeque::new();
        q.push_back((0, 0));
        while let Some((u, u_p)) = q.pop_front() {
            for &(v, _) in e[u].iter() {
                if v == u_p {
                    continue;
                }
                p[v] = u;
                q.push_back((v, u));
            }
        }
        p
    };

    let mut c = vec![0_i64; n];
    let mut d = vec![0_i64; n - 1];
    for &(t_i, e_i, x_i) in tex.iter() {
        match t_i {
            0 => {
                let (a_i, b_i) = ab[e_i];
                if e[a_i].len() == 1 {
                    c[a_i] += x_i;
                } else if p[a_i] == b_i {
                    c[a_i] += x_i;
                    for &(v, v_e_i) in e[a_i].iter() {
                        if v == b_i {
                            continue;
                        }
                        d[v_e_i] += x_i;
                    }
                } else {
                    c[0] += x_i;
                    for &(_, v_e_i) in e[0].iter() {
                        d[v_e_i] += x_i;
                    }
                    d[e_i] -= x_i;
                }
            }
            1 => {
                let (b_i, a_i) = ab[e_i];
                if e[a_i].len() == 1 {
                    c[a_i] += x_i;
                } else if p[a_i] == b_i {
                    c[a_i] += x_i;
                    for &(v, v_e_i) in e[a_i].iter() {
                        if v == b_i {
                            continue;
                        }
                        d[v_e_i] += x_i;
                    }
                } else {
                    c[0] += x_i;
                    for &(_, v_e_i) in e[0].iter() {
                        d[v_e_i] += x_i;
                    }
                    d[e_i] -= x_i;
                }
            }
            _ => unreachable!(),
        }
    }

    dfs(&mut c, &d, &e, 0, 0, 0);

    for c_i in c {
        println!("{}", c_i);
    }
}
