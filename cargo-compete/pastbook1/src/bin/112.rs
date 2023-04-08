use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        uv: [(Usize1, Usize1); m],
        mut c: [usize; n],
    }

    let mut edges = vec![vec![]; n];
    for (u, v) in uv.iter().copied() {
        edges[u].push(v);
        edges[v].push(u);
    }

    for _ in 0..q {
        input! {
            t: usize,
            x: Usize1,
        }
        match t {
            1 => {
                println!("{}", c[x]);
                for v in edges[x].iter().copied() {
                    c[v] = c[x];
                }
            }
            2 => {
                input! {
                    y: usize
                }
                println!("{}", c[x]);
                c[x] = y;
            }
            _ => unreachable!(),
        }
    }
}
