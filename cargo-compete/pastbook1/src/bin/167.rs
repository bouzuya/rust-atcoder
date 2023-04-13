use proconio::{input, marker::Chars};

fn dfs(
    h: usize,
    w: usize,
    c: &[Vec<char>],
    used: &mut Vec<Vec<bool>>,
    s: (usize, usize),
    g: (usize, usize),
) -> bool {
    if s == g {
        return true;
    }

    let (i, j) = s;
    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (i as i64 + dr, j as i64 + dc);
        if !(0..h as i64).contains(&nr) || !(0..w as i64).contains(&nc) {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if c[nr][nc] == '#' || used[nr][nc] {
            continue;
        }
        used[nr][nc] = true;

        if dfs(h, w, c, used, (nr, nc), g) {
            return true;
        }
    }
    false
}

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars; h],
    }

    let (mut s, mut g) = ((0, 0), (0, 0));
    for i in 0..h {
        for j in 0..w {
            if c[i][j] == 's' {
                s = (i, j);
            } else if c[i][j] == 'g' {
                g = (i, j);
            }
        }
    }

    let mut used = vec![vec![false; w]; h];
    used[s.0][s.1] = true;
    let ans = dfs(h, w, &c, &mut used, s, g);
    println!("{}", if ans { "Yes" } else { "No" });
}
