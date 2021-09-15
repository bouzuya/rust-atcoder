use proconio::input;

fn dfs(w: usize, h: usize, z: &[Vec<usize>], b: &mut [Vec<bool>], x: usize, y: usize) {
    if b[y][x] {
        return;
    }

    b[y][x] = true;

    let dir = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    for (dy, dx) in dir {
        let (ny, nx) = (y as i64 + dy, x as i64 + dx);
        if !(0..h as i64).contains(&ny) || !(0..w as i64).contains(&nx) {
            continue;
        }
        let (ny, nx) = (ny as usize, nx as usize);
        if b[ny][nx] {
            continue;
        }
        if z[ny][nx] >= z[y][x] {
            continue;
        }
        dfs(w, h, z, b, nx, ny);
    }
}

fn main() {
    loop {
        input! {
            w: usize,
            h: usize,
            p: usize,
        }

        if w == 0 && h == 0 && p == 0 {
            break;
        }

        input! {
            z: [[usize; w]; h],
            xy: [(usize, usize); p],
        }

        let mut b = vec![vec![false; w]; h];
        for (x_i, y_i) in xy {
            dfs(w, h, &z, &mut b, x_i, y_i);
        }

        let mut count = 0;
        for i in 0..h {
            for j in 0..w {
                if b[i][j] {
                    count += 1;
                }
            }
        }

        println!("{}", count);
    }
}
