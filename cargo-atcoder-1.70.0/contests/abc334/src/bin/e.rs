use std::collections::HashSet;

use ac_library::{Dsu, ModInt998244353 as ModInt};
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut dsu = Dsu::new(h * w);
    let mut count_red = 0_usize;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] == '.' {
                count_red += 1;
                continue;
            }
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if s[nr][nc] == '#' {
                    dsu.merge(i * w + j, nr * w + nc);
                }
            }
        }
    }
    let components = dsu.groups().len() - count_red;

    let mut ans = ModInt::new(0);
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '.' {
                continue;
            }

            let mut set = HashSet::new();
            let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
            for (dr, dc) in dir {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    continue;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                if s[nr][nc] == '#' {
                    set.insert(dsu.leader(nr * w + nc));
                }
            }
            if set.is_empty() {
                ans += components + 1;
            } else {
                ans += components - (set.len() - 1);
            }
        }
    }

    ans /= count_red;
    println!("{}", ans);
}
