use proconio::input;
use proconio::marker::Usize1;
use std::cmp::min;

// Warshall–Floyd
fn warshall_floyd(e: &Vec<Vec<(usize, u64)>>, inf: u64) -> Vec<Vec<u64>> {
    let n = e.len();
    let mut d = vec![vec![inf; n]; n];
    for u in 0..n {
        for &(v, l) in e[u].iter() {
            d[u][v] = l;
        }
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                if d[j][k] > d[j][i] + d[i][k] {
                    d[j][k] = d[j][i] + d[i][k];
                }
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        uvl: [(Usize1, Usize1, u64); m],
    };
    let mut e1 = vec![];
    let mut e = vec![vec![]; n];
    for (u_i, v_i, l_i) in uvl {
        if u_i == 0 || v_i == 0 {
            e1.push((if u_i == 0 { v_i } else { u_i }, l_i));
        } else {
            e[u_i].push((v_i, l_i));
            e[v_i].push((u_i, l_i));
        }
    }

    let inf = 1_000_000_000;
    let d = warshall_floyd(&e, inf);
    let mut ans = inf;
    for &(s, l_s) in e1.iter() {
        for &(g, l_g) in e1.iter() {
            if s == g {
                continue;
            }
            ans = min(ans, l_s + d[s][g] + l_g);
        }
    }
    println!("{}", if ans == inf { -1 } else { ans as i64 });
}
