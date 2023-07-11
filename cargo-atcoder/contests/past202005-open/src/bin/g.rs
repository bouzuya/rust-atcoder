use proconio::input;

fn main() {
    input! {
        n: usize,
        x: i64,
        y: i64,
        xy: [(i64, i64); n],
    };
    let mut d = vec![vec![(false, None); 402 + 1]; 402 + 1];
    let g_x = (x + 201) as usize;
    let g_y = (y + 201) as usize;
    for &(x_i, y_i) in xy.iter() {
        d[(y_i + 201) as usize][(x_i + 201) as usize].0 = true;
    }

    let mut q = std::collections::VecDeque::new();
    q.push_back((0, 201, 201));
    d[201][201].1 = Some(0);
    while let Some((d_u, d_y, d_x)) = q.pop_front() {
        let dir = vec![(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)];
        for (dx, dy) in dir {
            let ny = d_y as i64 + dy;
            let nx = d_x as i64 + dx;
            if !(0..=402).contains(&ny) {
                continue;
            }
            if !(0..=402).contains(&nx) {
                continue;
            }
            let ny = ny as usize;
            let nx = nx as usize;
            if d[ny][nx].0 {
                continue;
            }
            match d[ny][nx].1 {
                None => {}
                Some(d_old) => {
                    if d_old <= d_u + 1 {
                        continue;
                    }
                }
            }
            d[ny][nx].1 = Some(d_u + 1);
            q.push_back((d_u + 1, ny, nx));
        }
    }

    let ans = d[g_y][g_x].1.unwrap_or(-1);
    println!("{}", ans);
}
