use proconio::input;
use proconio::marker::{Chars, Usize1};

fn bfs(
    c: &Vec<Vec<char>>,
    h: usize,
    w: usize,
    s: (usize, usize),
    g: (usize, usize),
) -> Option<usize> {
    let ds = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    let inf = h * w + 1;
    let mut d = vec![vec![inf; w]; h];
    let mut deque = std::collections::VecDeque::new();
    d[s.1][s.0] = 0;
    deque.push_back(s);
    while let Some((y, x)) = deque.pop_front() {
        if (y, x) == g {
            return Some(d[y][x]);
        }
        for &(dy, dx) in ds.iter() {
            let ny = (y as i64 + dy) as usize;
            let nx = (x as i64 + dx) as usize;
            if c[ny][nx] == '.' && d[ny][nx] == inf {
                d[ny][nx] = d[y][x] + 1;
                deque.push_back((ny, nx));
            }
        }
    }
    None
}

fn main() {
    input! {
        h: usize,
        w: usize,
        s: (Usize1, Usize1),
        g: (Usize1, Usize1),
        c: [Chars; h],
    };
    match bfs(&c, h, w, s, g) {
        Some(ans) => println!("{}", ans),
        None => unreachable!(),
    }
}
