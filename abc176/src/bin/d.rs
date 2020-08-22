use proconio::input;
use proconio::marker::{Chars, Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        c_h: Usize1,
        c_w: Usize1,
        d_h: Usize1,
        d_w: Usize1,
        s: [Chars; h]
    };

    let inf = 1_000_000_000_001_i64;
    let mut d = vec![vec![inf; w]; h];
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((0, c_h, c_w)));
    d[c_h][c_w] = 0;
    while let Some(std::cmp::Reverse((t_d, t_h, t_w))) = pq.pop() {
        if d[t_h][t_w] < t_d {
            continue;
        }

        let dhw = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
        for (d_h, d_w) in dhw.iter() {
            let u_h = t_h as i64 + d_h;
            let u_w = t_w as i64 + d_w;
            if (0..h as i64).contains(&u_h) && (0..w as i64).contains(&u_w) {
                let u_h = u_h as usize;
                let u_w = u_w as usize;
                let u_d = t_d;
                if s[u_h][u_w] == '.' && d[u_h][u_w] > u_d {
                    d[u_h][u_w] = u_d;
                    pq.push(std::cmp::Reverse((u_d, u_h, u_w)));
                }
            }
        }
        {
            let t_h = t_h as i64;
            let t_w = t_w as i64;
            for u_h in t_h - 2..=t_h + 2 {
                for u_w in t_w - 2..=t_w + 2 {
                    let u_d = t_d + 1;
                    if (0..h as i64).contains(&u_h) && (0..w as i64).contains(&u_w) {
                        let u_h = u_h as usize;
                        let u_w = u_w as usize;
                        if s[u_h][u_w] == '.' && d[u_h][u_w] > u_d {
                            d[u_h][u_w] = u_d;
                            pq.push(std::cmp::Reverse((u_d, u_h, u_w)));
                        }
                    }
                }
            }
        }
    }

    if d[d_h][d_w] == inf {
        println!("-1");
    } else {
        println!("{}", d[d_h][d_w]);
    }
}
