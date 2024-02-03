use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    };
    let mut board = vec![vec!['.'; w]; h];
    let mut cur = (0_usize, 0_usize);
    let mut dir = 0;
    let dirs = vec![(-1, 0), (0, 1), (1, 0), (0, -1)];
    for _ in 0..n {
        match board[cur.0][cur.1] {
            '.' => {
                board[cur.0][cur.1] = '#';
                dir = (dir + 1) % 4;
                let (ci, cj) = cur;
                let (di, dj) = dirs[dir];
                let ni = ((ci + h) as i64 + di) as usize % h;
                let nj = ((cj + w) as i64 + dj) as usize % w;
                cur = (ni, nj);
            }
            '#' => {
                board[cur.0][cur.1] = '.';
                dir = (4 + dir - 1) % 4;
                let (ci, cj) = cur;
                let (di, dj) = dirs[dir];
                let ni = ((ci + h) as i64 + di) as usize % h;
                let nj = ((cj + w) as i64 + dj) as usize % w;
                cur = (ni, nj);
            }
            _ => unreachable!(),
        }
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", board[i][j]);
        }
        println!();
    }
}
