use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };

    // bfs
    let inf = 1_000_000_000;
    let dydx = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut d = vec![vec![inf; w]; h];
    let mut q = std::collections::VecDeque::new();
    d[0][0] = 1;
    q.push_back((0, 0));
    while let Some((y, x)) = q.pop_front() {
        for &(dy, dx) in dydx.iter() {
            let (ny, nx) = (y as i64 + dy, x as i64 + dx);
            if (0..h as i64).contains(&ny)
                && (0..w as i64).contains(&nx)
                && d[ny as usize][nx as usize] == inf
                && s[ny as usize][nx as usize] == '.'
            {
                d[ny as usize][nx as usize] = d[y][x] + 1;
                q.push_back((ny as usize, nx as usize));
            }
        }
    }
    if d[h - 1][w - 1] == inf {
        println!("-1");
    } else {
        let mut c_w = 0;
        for y in 0..h {
            for x in 0..w {
                if s[y][x] == '.' {
                    c_w += 1;
                }
            }
        }
        println!("{}", c_w - d[h - 1][w - 1]);
    }
}
