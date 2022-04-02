use std::{cmp::Reverse, collections::VecDeque};

use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        a: (Usize1, Usize1),
        b: (Usize1, Usize1),
        s: [Chars; n],
    };
    let inf = n * n + 1;
    let mut dist = vec![vec![inf; n]; n];

    let mut pq = VecDeque::new();
    dist[a.0][a.1] = 0;
    pq.push_back((Reverse(0), a.0, a.1, 0));
    pq.push_back((Reverse(0), a.0, a.1, 1));
    while let Some((Reverse(c_d), i, j, k)) = pq.pop_front() {
        if dist[i][j] != c_d {
            continue;
        }

        let dir = vec![(-1, -1, 0), (-1, 1, 1), (1, -1, 1), (1, 1, 0)];
        for (dr, dc, dd) in dir {
            if dd == k {
                continue;
            }
            let nd = c_d + 1;
            for l in 1.. {
                let (nr, nc) = (i as i64 + dr * l, j as i64 + dc * l);
                if !(0..n as i64).contains(&nr) || !(0..n as i64).contains(&nc) {
                    break;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if s[nr][nc] == '#' {
                    break;
                }
                if nd < dist[nr][nc] {
                    dist[nr][nc] = nd;
                    pq.push_back((Reverse(nd), nr, nc, dd));
                } else if nd >= dist[nr][nc] + 1 {
                    break;
                }
            }
        }
    }

    let v = dist[b.0][b.1];
    let ans = if v == inf { -1 } else { v as i64 };
    println!("{}", ans);
}
