use proconio::input;
use proconio::marker::Usize1;
use superslice::*;

enum Op {
    Add,
    Replace(usize, u64),
}

fn dfs(
    ls: &mut Vec<usize>,
    lis: &mut Vec<u64>,
    stack: &mut Vec<Op>,
    es: &Vec<Vec<usize>>,
    ais: &Vec<u64>,
    u: usize,
    p: usize,
) {
    for &v in es[u].iter() {
        if v == p {
            continue;
        }

        let a = ais[v];
        let &last_a = lis.last().unwrap();
        if last_a < a {
            lis.push(a);
            stack.push(Op::Add);
        } else {
            let i = lis[..].lower_bound(&a);
            let b = lis[i];
            lis[i] = a;
            stack.push(Op::Replace(i, b));
        }
        ls[v] = lis.len();

        dfs(ls, lis, stack, es, ais, v, u);

        match stack.pop().unwrap() {
            Op::Add => {
                lis.pop();
            }
            Op::Replace(i, b) => {
                lis[i] = b;
            }
        }
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
    let mut lis = vec![];
    let mut stack = vec![];
    ls[0] = 1;
    stack.push(Op::Add);
    lis.push(ais[0]);
    dfs(&mut ls, &mut lis, &mut stack, &es, &ais, 0, 0);

    for l in ls {
        println!("{}", l);
    }
}
