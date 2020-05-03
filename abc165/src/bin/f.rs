use proconio::input;
use proconio::marker::Usize1;
use superslice::*;

fn dfs(
    ls: &mut Vec<usize>,
    lis: &mut Vec<u64>,
    es: &Vec<Vec<usize>>,
    ais: &Vec<u64>,
    u: usize,
    p: usize,
) {
    let a = ais[u];
    let b = if *(lis.last().unwrap_or(&0)) < a {
        lis.push(a);
        None
    } else {
        let i = lis[..].lower_bound(&a);
        let x = lis[i];
        lis[i] = a;
        Some((i, x))
    };
    ls[u] = lis.len();
    for &v in es[u].iter() {
        if v == p {
            continue;
        }
        dfs(ls, lis, es, ais, v, u);
    }
    match b {
        None => {
            lis.pop();
        }
        Some((i, x)) => lis[i] = x,
    }
}

fn main() {
    input! {
        n: usize,
        ais: [u64; n],
        uvs: [(Usize1, Usize1); n - 1],
    };

    let mut es = vec![vec![]; n];
    for (u, v) in uvs {
        es[u].push(v);
        es[v].push(u);
    }

    let mut ls = vec![0_usize; n];
    dfs(&mut ls, &mut vec![], &es, &ais, 0, 0);

    for l in ls {
        println!("{}", l);
    }
}
