use proconio::input;
use proconio::marker::Usize1;
use std::collections::VecDeque;

fn topological_sort(e: &Vec<Vec<usize>>) -> Option<Vec<usize>> {
    let n = e.len();
    let mut c_in = vec![0; n];
    for e_u in e.iter() {
        for &v in e_u.iter() {
            c_in[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    for u in 0..n {
        if c_in[u] == 0 {
            q.push_back(u);
        }
    }
    let mut res = vec![];
    while let Some(u) = q.pop_front() {
        res.push(u);
        for &v in e[u].iter() {
            c_in[v] -= 1;
            if c_in[v] == 0 {
                q.push_back(v);
            }
        }
    }
    if res.len() == n {
        Some(res)
    } else {
        None
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        lrd: [(Usize1, Usize1, u64); m],
    };

    let mut e = vec![vec![]; n];
    let mut e2 = vec![vec![]; n];
    for &(l_i, r_i, d_i) in lrd.iter() {
        e[l_i].push(r_i);
        e2[l_i].push((r_i, d_i));
    }

    let sorted = match topological_sort(&e) {
        None => {
            println!("No");
            return;
        }
        Some(sorted) => sorted,
    };

    let mut d = vec![0; n];
    d[sorted[0]] = 0;
    for &u in sorted.iter() {
        let p = d[u];
        for &(v_i, d_i) in e2[u].iter() {
            if d[v_i] == 0 {
                d[v_i] = p + d_i;
            }
            if d[v_i] != p + d_i {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
