use std::collections::BinaryHeap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut c: [Chars; h],
    };

    let mut heap = BinaryHeap::new();
    let mut count = vec![vec![0; w]; h];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] != '.' {
                count[i][j] = 9;
                continue;
            }
            let mut count_ij = 0;
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir.iter().copied() {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    count_ij += 1;
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if c[nr][nc] == '.' {
                    continue;
                }
                count_ij += 1;
            }
            count[i][j] = count_ij;
            heap.push((count_ij, i, j));
        }
    }

    while let Some((count_ij, i, j)) = heap.pop() {
        if count[i][j] != count_ij {
            continue;
        }

        let mut b = vec![false; 5];
        let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (dr, dc) in dir {
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if c[nr][nc] == '.' {
                count[nr][nc] += 1;
                heap.push((count[nr][nc], nr, nc));
                continue;
            }
            b[(c[nr][nc] as u8 - b'1') as usize] = true;
        }

        for (k, b_k) in b.into_iter().enumerate() {
            if b_k {
                continue;
            }

            c[i][j] = (b'1' + k as u8) as char;
        }
    }
    for i in 0..h {
        for j in 0..w {
            print!("{}", c[i][j]);
        }
        println!();
    }
    // let ans = n - a.len();
    // println!("{}", ans);
}
