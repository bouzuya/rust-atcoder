use std::{cmp, collections::BinaryHeap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut s = (0, 0);
    let mut g = (0, 0);
    for i in 0..h {
        for j in 0..w {
            match c[i][j] {
                's' => s = (i, j),
                'g' => g = (i, j),
                _ => {}
            }
        }
    }

    let inf = h * w + 1;
    let mut d = vec![vec![inf; w]; h];
    let mut q = BinaryHeap::new();
    q.push((cmp::Reverse(0), s.0, s.1));
    d[s.0][s.1] = 0;
    while let Some((cmp::Reverse(d_ij), i, j)) = q.pop() {
        if d_ij > d[i][j] {
            continue;
        }
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            let nd = d_ij
                + match c[nr][nc] {
                    's' => 0,
                    'g' => 0,
                    '.' => 0,
                    '#' => 1,
                    _ => unreachable!(),
                };
            if nd < d[nr][nc] {
                q.push((cmp::Reverse(nd), nr, nc));
                d[nr][nc] = nd;
            }
        }
    }

    let ans = d[g.0][g.1] <= 2;
    println!("{}", if ans { "YES" } else { "NO" });
}
