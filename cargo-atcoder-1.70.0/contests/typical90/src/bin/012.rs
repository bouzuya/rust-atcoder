use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        capital_q: usize,
    };

    let mut dsu = Dsu::new(h * w);
    let mut table = vec![vec![false; w]; h];

    for _ in 0..capital_q {
        input! {
            t: usize,
        };

        match t {
            1 => {
                input! {
                    r: Usize1,
                    c: Usize1,
                }
                table[r][c] = true;
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dr, dc) in dir {
                    let (nr, nc) = (r as i64 + dr, c as i64 + dc);
                    if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                        continue;
                    }
                    let (nr, nc) = (nr as usize, nc as usize);
                    if table[nr][nc] {
                        dsu.merge(r * w + c, nr * w + nc);
                    }
                }
            }
            2 => {
                input! {
                    ra: Usize1,
                    ca: Usize1,
                    rb: Usize1,
                    cb: Usize1,
                }
                let ans = table[ra][ca] && dsu.same(ra * w + ca, rb * w + cb);
                println!("{}", if ans { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
