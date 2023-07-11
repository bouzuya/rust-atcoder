use std::collections::VecDeque;

use proconio::{
    input,
    marker::{Bytes, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        r_s: Usize1,
        c_s: Usize1,
        r_t: Usize1,
        c_t: Usize1,
        s: [Bytes; h],
    };
    let inf = 1_000_000_000;
    let mut d = vec![vec![vec![inf; 4]; w]; h];
    let mut q = VecDeque::new();
    for i in 0..4 {
        q.push_front((0, i, r_s, c_s));
        d[r_s][c_s][i] = 0;
    }
    while let Some((d_i, dir_i, r_i, c_i)) = q.pop_front() {
        if d[r_i][c_i][dir_i] != d_i {
            continue;
        }
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (i, &(dr, dc)) in dir.iter().enumerate() {
            let (nr, nc) = (r_i as i64 + dr, c_i as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if s[nr][nc] == b'#' {
                continue;
            }
            let nd = d_i + if dir_i == i { 0 } else { 1 };
            if nd < d[nr][nc][i] {
                d[nr][nc][i] = nd;
                if d_i == nd {
                    q.push_front((nd, i, nr, nc));
                } else {
                    q.push_back((nd, i, nr, nc));
                }
            }
        }
    }

    let ans = *d[r_t][c_t].iter().min().unwrap();
    println!("{}", ans);
}
