use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        n: usize,
        q: usize,
        ftx: [(Usize1, Usize1, Usize1); q],
    };
    // コンテナを 0..n 、机を n..2n で表現する。
    let mut e = vec![(0, 0); 2 * n]; // (from, to)
    for i in 0..n {
        e[i].0 = n + i;
        e[i].1 = n + i;
    }
    for i in 0..n {
        e[n + i].0 = i;
        e[n + i].1 = i;
    }
    for &(f_i, t_i, x_i) in ftx.iter() {
        let x_f = x_i;
        let x_t = e[n + f_i].0;

        e[x_t].1 = n + t_i;
        e[n + f_i].0 = e[x_f].0;
        let t = e[x_f].0;
        e[t].1 = n + f_i;

        e[x_f].0 = e[n + t_i].0;
        e[x_t].1 = n + t_i;

        let t = e[n + t_i].0;
        e[t].1 = x_f;
        e[n + t_i].0 = x_t;
    }
    let mut ans = vec![0; n];
    for i in n..2 * n {
        let mut c = e[i].1;
        while c != i {
            ans[c] = i - n + 1;
            c = e[c].1;
        }
    }
    for &a_i in ans.iter() {
        println!("{}", a_i);
    }
}
