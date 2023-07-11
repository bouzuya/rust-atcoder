use proconio::{input, marker::Chars};
use std::cmp;

fn dfs(
    h: usize,
    w: usize,
    cell: &[Vec<char>],
    start: (usize, usize),
    pos: (usize, usize),
    dist: usize,
    used: &mut [Vec<bool>],
) -> usize {
    let (i, j) = pos;
    let mut max = 0;
    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
            continue;
        }
        let (nr, nc, nd) = (nr as usize, nc as usize, dist + 1);
        if cell[nr][nc] == '#' {
            continue;
        }
        if start == (nr, nc) {
            max = cmp::max(max, nd);
            continue;
        }
        if used[nr][nc] {
            continue;
        }
        used[nr][nc] = true;
        max = cmp::max(max, dfs(h, w, cell, start, (nr, nc), nd, used));
        used[nr][nc] = false;
    }
    max
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };
    let mut max = 0;
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                let mut used = vec![vec![false; w]; h];
                used[i][j] = true;
                let pos = (i, j);
                max = cmp::max(max, dfs(h, w, &c, pos, pos, 0, &mut used));
            }
        }
    }
    let ans = if max <= 3 { -1 } else { max as i64 };
    println!("{}", ans);
}
