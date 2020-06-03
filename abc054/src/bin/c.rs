use proconio::input;
use proconio::marker::Usize1;
use superslice::*;

fn adjacency_list(n: usize, uv: &Vec<(usize, usize)>) -> Vec<Vec<usize>> {
    let mut e = vec![vec![]; n];
    for &(u, v) in uv.iter() {
        e[u].push(v);
        e[v].push(u);
    }
    e
}

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(Usize1, Usize1); m],
    };
    let e = adjacency_list(n, &ab);
    let mut vs = (1..n).collect::<Vec<usize>>();
    let mut count = 0;
    loop {
        let mut is_ok = true;
        let mut u = 0;
        for &v_i in vs.iter() {
            if !e[u].contains(&v_i) {
                is_ok = false;
                break;
            }
            u = v_i;
        }
        if is_ok {
            count += 1;
        }
        if !vs.next_permutation() {
            break;
        }
    }
    let ans = count;
    println!("{}", ans);
}
