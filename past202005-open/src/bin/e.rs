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
        mut c: [i64; n],
    };

    let e = adjacency_list(n, &uv);

    for _ in 0..q {
        input! {
            t_i: usize,
        };
        match t_i {
            1 => {
                input! {
                    x_i: Usize1,
                };
                println!("{}", c[x_i]);
                for &v in e[x_i].iter() {
                    c[v] = c[x_i];
                }
            }
            2 => {
                input! {
                    x_i: Usize1,
                    y_i: i64,
                };
                println!("{}", c[x_i]);
                c[x_i] = y_i;
            }
            _ => unreachable!(),
        }
    }
}
