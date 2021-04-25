use std::collections::VecDeque;

use proconio::input;
use proconio::marker::Chars;

fn shortest_path(
    n: usize,
    m: usize,
    s: &Vec<Vec<char>>,
    (i, j): (usize, usize),
) -> Vec<Vec<Option<usize>>> {
    let mut d = vec![vec![None; m]; n];
    let mut q = VecDeque::new();
    d[i][j] = Some(0);
    q.push_back((i, j, 0));
    while let Some((i, j, d_ij)) = q.pop_front() {
        if d[i][j] != Some(d_ij) {
            continue;
        }
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (di, dj) in dir {
            let (ni, nj) = (i as i64 + di, j as i64 + dj);
            if !(0..n as i64).contains(&ni) {
                continue;
            }
            if !(0..m as i64).contains(&nj) {
                continue;
            }
            let (ni, nj) = (ni as usize, nj as usize);
            if s[ni][nj] == '#' {
                continue;
            }
            match d[ni][nj] {
                None => {
                    d[ni][nj] = Some(d_ij + 1);
                    q.push_back((ni, nj, d_ij + 1));
                }
                Some(_) => continue,
            }
        }
    }
    d
}

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    };
    let mut p_c = (n, m);
    let mut p_s = (n, m);
    let mut p_g = (n, m);
    for i in 0..n {
        for j in 0..m {
            match s[i][j] {
                'C' => p_c = (i, j),
                'S' => p_s = (i, j),
                'G' => p_g = (i, j),
                '.' | '#' => {}
                _ => unreachable!(),
            }
        }
    }

    let d = shortest_path(n, m, &s, p_s);
    let d_c = match d[p_c.0][p_c.1] {
        None => {
            println!("-1");
            return;
        }
        Some(d_c) => d_c,
    };
    let d = shortest_path(n, m, &s, p_c);
    let d_g = match d[p_g.0][p_g.1] {
        None => {
            println!("-1");
            return;
        }
        Some(d_g) => d_g,
    };
    let ans = d_c + d_g;
    println!("{}", ans);
}
