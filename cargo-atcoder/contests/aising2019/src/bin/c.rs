// 解説 AC
use proconio::input;
use proconio::marker::Chars;
use std::collections::VecDeque;

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars; h],
    };
    let mut b = vec![vec![false; w]; h];
    let mut count = 0_i64;
    for i in 0..h {
        for j in 0..w {
            if b[i][j] {
                continue;
            }
            let mut count_b = 0;
            let mut count_w = 0;
            b[i][j] = true;
            let mut deque = VecDeque::new();
            deque.push_back((i, j));
            while let Some((y, x)) = deque.pop_front() {
                if s[y][x] == '#' {
                    count_b += 1;
                } else {
                    count_w += 1;
                }
                let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
                for (dy, dx) in dir {
                    let (ny, nx) = (y as i64 + dy, x as i64 + dx);
                    if !(0..h as i64).contains(&ny) || !(0..w as i64).contains(&nx) {
                        continue;
                    }
                    let (ny, nx) = (ny as usize, nx as usize);
                    if s[y][x] == s[ny][nx] || b[ny][nx] {
                        continue;
                    }
                    b[ny][nx] = true;
                    deque.push_back((ny, nx));
                }
            }
            count += count_b * count_w;
        }
    }
    let ans = count;
    println!("{}", ans);
}
