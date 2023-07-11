use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        k: i64,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        c: [Chars; h],
    };
    let mut d = vec![vec![None; w]; h];
    let mut q = std::collections::VecDeque::new();
    d[s.0][s.1] = Some(0);
    q.push_back((0, s.0, s.1));
    while let Some((cost, y, x)) = q.pop_front() {
        for &(dy, dx) in [(-1, 0), (0, 1), (1, 0), (0, -1)].iter() {
            for k_i in 1..=k {
                let (ny, nx, ncost) = (
                    dy as i64 * k_i + y as i64,
                    dx as i64 * k_i + x as i64,
                    cost + 1,
                );
                if (0..h as i64).contains(&ny) && (0..w as i64).contains(&nx) {
                    let (ny, nx) = (ny as usize, nx as usize);
                    if c[ny][nx] == '@' {
                        break;
                    }
                    match d[ny][nx] {
                        Some(w_o) if ncost > w_o => break,
                        Some(w_o) if ncost == w_o => continue,
                        None | Some(_) => {
                            d[ny][nx] = Some(ncost);
                            q.push_back((ncost, ny, nx));
                        }
                    }
                }
            }
        }
    }
    println!("{}", d[g.0][g.1].unwrap_or(-1));
}
