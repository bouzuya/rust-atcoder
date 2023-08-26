use std::collections::VecDeque;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        a: [Chars; h],
    };

    let mut s = (0, 0);
    let mut g = (0, 0);
    let mut ok = vec![vec![true; w]; h];
    for i in 0..h {
        for j in 0..w {
            match a[i][j] {
                'S' => s = (i, j),
                'G' => g = (i, j),
                '.' => {}
                '#' | '>' | 'v' | '<' | '^' => {
                    ok[i][j] = false;
                }
                _ => unreachable!(),
            }
        }
    }

    for i in 0..h {
        let mut ng = false;
        for j in 0..w {
            if ng {
                match a[i][j] {
                    '.' => {
                        ok[i][j] = false;
                    }
                    '#' | 'v' | '<' | '^' => ng = false,
                    '>' => ng = true,
                    _ => {}
                }
            } else if a[i][j] == '>' {
                ng = true;
            }
        }

        let mut ng = false;
        for j in (0..w).rev() {
            if ng {
                match a[i][j] {
                    '.' => {
                        ok[i][j] = false;
                    }
                    '#' | '>' | 'v' | '^' => ng = false,
                    '<' => ng = true,
                    _ => {}
                }
            } else if a[i][j] == '<' {
                ng = true;
            }
        }
    }

    for j in 0..w {
        let mut ng = false;
        for i in 0..h {
            if ng {
                match a[i][j] {
                    '.' => {
                        ok[i][j] = false;
                    }
                    '#' | '>' | '<' | '^' => ng = false,
                    'v' => ng = true,
                    _ => {}
                }
            } else if a[i][j] == 'v' {
                ng = true;
            }
        }

        let mut ng = false;
        for i in (0..h).rev() {
            if ng {
                match a[i][j] {
                    '.' => {
                        ok[i][j] = false;
                    }
                    '#' | '>' | '<' | 'v' => ng = false,
                    '^' => ng = true,
                    _ => {}
                }
            } else if a[i][j] == '^' {
                ng = true;
            }
        }
    }

    // let tbl = &ok;
    // for i in 0..h {
    //     for j in 0..w {
    //         print!("{}", if tbl[i][j] { '.' } else { '#' });
    //     }
    //     println!();
    // }
    // println!();

    let inf = 1_usize << 60;
    let mut dist = vec![vec![inf; w]; h];
    let mut q = VecDeque::new();
    q.push_back(s);
    dist[s.0][s.1] = 0;
    while let Some((i, j)) = q.pop_front() {
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if !ok[nr][nc] {
                continue;
            }
            if dist[nr][nc] != inf {
                continue;
            }
            dist[nr][nc] = dist[i][j] + 1;
            q.push_back((nr, nc));
        }
    }

    let ans = if dist[g.0][g.1] == inf {
        -1
    } else {
        dist[g.0][g.1] as i64
    };
    println!("{}", ans);
}
