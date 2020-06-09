use proconio::input;
use proconio::marker::Usize1;

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
        q: usize,
        uv: [(Usize1, Usize1); m],
        mut c: [u64; n],
    };

    let e = adjacency_list(n, &uv);
    for _ in 0..q {
        input! {
            t: usize,
            x: Usize1,
        };
        match t {
            1 => {
                println!("{}", c[x]);
                for &v in e[x].iter() {
                    c[v] = c[x];
                }
            }
            2 => {
                input! { y: u64 };
                println!("{}", c[x]);
                c[x] = y;
            }
            _ => unreachable!(),
        }
    }
}
