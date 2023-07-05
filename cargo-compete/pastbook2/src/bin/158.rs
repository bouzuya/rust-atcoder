use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: [Chars; 4],
    }

    let (h, w) = (4, 4);
    let to_bit = |i: usize, j: usize| -> usize { 1 << (i * w + j) };
    let grid = {
        let mut grid = 0_usize;
        for i in 0..h {
            for j in 0..w {
                if s[i][j] == '#' {
                    grid |= to_bit(i, j);
                }
            }
        }
        grid
    };

    let mut dp = vec![std::f64::MAX; 1 << (h * w)];
    dp[0] = 0_f64;
    for s in 0..(1 << (h * w)) {
        for i in 0..h {
            for j in 0..w {
                let mut ps = vec![];

                let dir = vec![(-1, 0), (0, -1), (0, 0), (0, 1), (1, 0)];
                for (dr, dc) in dir.iter().copied() {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if (s & to_bit(nr, nc)) == 0 {
                        continue;
                    }

                    ps.push(s ^ to_bit(nr, nc));
                }

                if ps.is_empty() {
                    continue;
                }

                dp[s] = dp[s].min(
                    ps.iter().copied().map(|p| dp[p]).sum::<f64>() / ps.len() as f64
                        + dir.len() as f64 / ps.len() as f64,
                );
            }
        }
    }

    let ans = dp[grid];
    println!("{}", ans);
}
