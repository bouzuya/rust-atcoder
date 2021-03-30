use proconio::input;
use proconio::marker::{Chars, Usize1};
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        sx: Usize1,
        sy: Usize1,
        gx: Usize1,
        gy: Usize1,
        s: [Chars; h],
    };
    let inf = h * w + 1;
    let mut d = vec![vec![vec![inf; 2]; w]; h];
    let mut q = VecDeque::new();
    q.push_back((sx, sy, 0)); // v = 0 // |
    q.push_back((sx, sy, 1)); // v = 1 // -
    d[sx][sy][0] = 0;
    d[sx][sy][1] = 0;
    while let Some((x, y, v)) = q.pop_front() {
        let dir = vec![(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (i, &(dx, dy)) in dir.iter().enumerate() {
            match (v == 0, i % 2 == 0) {
                (false, false) | (true, true) => {
                    let (nx, ny) = (x as i64 + dx, y as i64 + dy);
                    if !(0..h as i64).contains(&nx) {
                        continue;
                    }
                    if !(0..w as i64).contains(&ny) {
                        continue;
                    }
                    let (nx, ny, nv) = (nx as usize, ny as usize, (v + 1) % 2);
                    if s[nx][ny] == '#' {
                        continue;
                    }
                    if d[nx][ny][nv] != inf {
                        continue;
                    }
                    d[nx][ny][nv] = d[x][y][v] + 1;
                    q.push_back((nx, ny, nv));
                }
                _ => {
                    continue;
                }
            }
        }
    }
    let ans = *d[gx][gy].iter().min().unwrap();
    println!("{}", if ans == inf { -1 } else { ans as i64 });
}
