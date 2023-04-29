use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    };

    let n = h.min(w);
    let mut x = vec![vec![0_usize; w]; h];
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '.' {
                continue;
            }
            let mut ok = true;
            for k in 1..=n as i64 {
                let dir = vec![(k, k), (k, -k), (-k, k), (-k, -k)];
                for (dr, dc) in dir {
                    let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                    if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                        ok = false;
                        break;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if c[nr][nc] == '.' {
                        ok = false;
                        break;
                    }
                }
                if ok {
                    x[i][j] = k as usize;
                } else {
                    break;
                }
            }
        }
    }

    let mut ans = vec![0_usize; n + 1];
    for i in 0..h {
        for j in 0..w {
            ans[x[i][j]] += 1;
        }
    }

    for a in 1..=n {
        println!("{}", ans[a]);
    }
}
