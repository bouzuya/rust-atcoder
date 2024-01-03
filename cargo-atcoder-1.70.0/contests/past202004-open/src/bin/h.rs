use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [Chars; n],
    };
    let mut b = vec![vec![]; 11];
    for i in 0..n {
        for j in 0..m {
            match a[i][j] {
                'S' => {
                    b[0] = vec![(i, j)];
                }
                'G' => {
                    b[10] = vec![(i, j)];
                }
                _ => {
                    b[(a[i][j] as u8 - b'0') as usize].push((i, j));
                }
            }
        }
    }

    let inf = 1_000_000_000_i64;

    let mut dist = vec![vec![inf; m]; n];
    dist[b[0][0].0][b[0][0].1] = 0;
    for i in 1..=10 {
        let mut next = vec![vec![inf; m]; n];
        for (pr, pc) in b[i - 1].iter().copied() {
            let pd = dist[pr][pc];
            for (nr, nc) in b[i].iter().copied() {
                let nd = pd + (pr as i64 - nr as i64).abs() + (pc as i64 - nc as i64).abs();
                next[nr][nc] = next[nr][nc].min(nd);
            }
        }
        dist = next;
    }

    let ans = dist[b[10][0].0][b[10][0].1];
    let ans = if ans == inf { -1 } else { ans };
    println!("{}", ans);
}
