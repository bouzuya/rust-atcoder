use proconio::input;
use proconio::marker::Chars;

fn dfs(
    h: usize,
    w: usize,
    s: &Vec<Vec<char>>,
    count: usize,
    r: usize,
    c: usize,
    route: &mut Vec<(usize, usize)>,
    used: &mut Vec<Vec<bool>>,
) -> bool {
    used[r][c] = true;
    route.push((r, c));
    if count == route.len() {
        let mut ok = true;
        for r in 0..h {
            for c in 0..w {
                if !used[r][c] && s[r][c] == '#' {
                    ok = false;
                    break;
                }
            }
        }
        return ok;
    }

    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dr, dc) in dir {
        let (nr, nc) = (r as i64 + dr, c as i64 + dc);
        if !(0..h as i64).contains(&nr) {
            continue;
        }
        if !(0..w as i64).contains(&nc) {
            continue;
        }
        let (nr, nc) = (nr as usize, nc as usize);
        if s[nr][nc] == '.' || used[nr][nc] {
            continue;
        }
        if dfs(h, w, s, count, nr, nc, route, used) {
            return true;
        }
        route.pop();
        used[nr][nc] = false;
    }
    false
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut count = 0;
    for r in 0..h {
        for c in 0..w {
            if s[r][c] == '#' {
                count += 1;
            }
        }
    }
    for r in 0..h {
        for c in 0..w {
            if s[r][c] != '#' {
                continue;
            }
            let mut route = vec![];
            let mut used = vec![vec![false; w]; h];
            if dfs(h, w, &s, count, r, c, &mut route, &mut used) {
                println!("{}", route.len());
                for &(r, c) in route.iter() {
                    println!("{} {}", r + 1, c + 1);
                }
                return;
            }
        }
    }
    unreachable!("invalid input");
}
