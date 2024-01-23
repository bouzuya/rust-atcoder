use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        ftx: [(Usize1, Usize1, Usize1); q],
    };

    let mut cb = vec![n; n];
    let mut dt = (0..n).collect::<Vec<usize>>();

    for (f, t, x) in ftx {
        let (dt_f, dt_t, cb_x) = (dt[f], dt[t], cb[x]);
        (dt[f], dt[t], cb[x]) = (cb_x, dt_f, dt_t);
    }
    let mut ans = vec![n; n];
    for i in 0..n {
        let mut p = dt[i];
        while p < n {
            ans[p] = i;
            p = cb[p];
        }
    }

    for a in ans {
        println!("{}", a + 1);
    }
}
