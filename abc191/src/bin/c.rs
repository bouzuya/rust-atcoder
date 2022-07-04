use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h]
    };
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if s[i][j] != '.' {
                continue;
            }
            let e = vec![
                (-1, 0),
                (-1, 1),
                (0, 1),
                (1, 1),
                (1, 0),
                (1, -1),
                (0, -1),
                (-1, -1),
            ]
            .into_iter()
            .map(|(dr, dc)| {
                let (nr, nc) = (i as i64 + dr, j as i64 + dc);
                if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
                    return false;
                }
                let (nr, nc) = (nr as usize, nc as usize);
                s[nr][nc] == '#'
            })
            .collect::<Vec<bool>>();
            for i in (1..8).step_by(2) {
                if (!e[i - 1] && e[i] && !e[(i + 1) % 8]) || (e[i - 1] && e[i] && e[(i + 1) % 8]) {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
