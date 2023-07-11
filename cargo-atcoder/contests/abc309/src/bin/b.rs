use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };

    let mut cur = (0_usize, 0_usize);
    let mut pos = vec![];
    let dir = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    for (dr, dc) in dir {
        for _ in 0..n - 1 {
            let (i, j) = cur;
            let (nr, nc) = (i as i64 + dr, j as i64 + dc);
            let (nr, nc) = (nr as usize, nc as usize);
            cur = (nr, nc);
            pos.push(cur);
        }
    }

    let mut b = a.clone();
    for (i, (cr, cc)) in pos.iter().copied().enumerate() {
        let (nr, nc) = pos[(i + 1) % pos.len()];
        b[nr][nc] = a[cr][cc];
    }

    for i in 0..n {
        for j in 0..n {
            print!("{}", b[i][j]);
        }
        println!();
    }
}
