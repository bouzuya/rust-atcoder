use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(i64, i64, i64); n],
        def: [(i64, i64, i64); m]
    };
    let mut map_x = std::collections::BTreeMap::new();
    let mut map_y = std::collections::BTreeMap::new();
    for &(a, b, c) in abc.iter() {
        map_x.entry(a).or_insert(0);
        map_x.entry(b).or_insert(0);
        map_y.entry(c).or_insert(0);
    }
    for &(d, e, f) in def.iter() {
        map_x.entry(d).or_insert(0);
        map_y.entry(e).or_insert(0);
        map_y.entry(f).or_insert(0);
    }
    map_x.entry(0).or_insert(0);
    map_y.entry(0).or_insert(0);

    let mut xs = vec![0; map_x.len()];
    for (i, (&x, v)) in map_x.iter_mut().enumerate() {
        *v = i;
        xs[i] = x;
    }
    let mut ys = vec![0; map_y.len()];
    for (i, (&y, v)) in map_y.iter_mut().enumerate() {
        *v = i;
        ys[i] = y;
    }

    let h = map_x.len() * 2;
    let w = map_y.len() * 2;
    let mut tbl = vec![vec![Some(false); w]; h];
    for (a, b, c) in abc.iter() {
        let tbl_a = map_x.get(a).unwrap() * 2;
        let tbl_b = map_x.get(b).unwrap() * 2;
        let tbl_c = map_y.get(c).unwrap() * 2;
        (tbl_a..=tbl_b).for_each(|x| tbl[x][tbl_c] = None);
    }
    for (d, e, f) in def.iter() {
        let tbl_d = map_x.get(d).unwrap() * 2;
        let tbl_e = map_y.get(e).unwrap() * 2;
        let tbl_f = map_y.get(f).unwrap() * 2;
        (tbl_e..=tbl_f).for_each(|y| tbl[tbl_d][y] = None);
    }

    let d = vec![(-1, 0), (0, -1), (0, 1), (1, 0)];
    let mut q = std::collections::VecDeque::new();
    let (sx, sy) = (map_x.get(&0).unwrap() * 2, map_y.get(&0).unwrap() * 2);
    tbl[sx][sy] = Some(true);
    q.push_back((sx, sy));
    while let Some((x, y)) = q.pop_front() {
        for &(d_x, d_y) in d.iter() {
            let (nx, ny) = (x as i64 + d_x, y as i64 + d_y);
            if (0..w as i64).contains(&ny) && (0..h as i64).contains(&nx) {
                let (nx, ny) = (nx as usize, ny as usize);
                if tbl[nx][ny] != Some(false) {
                    continue;
                }
                tbl[nx][ny] = Some(true);
                q.push_back((nx, ny));
            }
        }
    }

    let mut ans = 0_i64;
    for x in 0..h {
        for y in 0..w {
            if tbl[x][y] != Some(true) {
                continue;
            }
            if x == 0 || x == h - 1 || y == 0 || y == w - 1 {
                println!("INF");
                return;
            }
            if x % 2 == 0 || y % 2 == 0 {
                continue;
            }
            let ex = xs[x / 2 + 1] - xs[x / 2];
            let ey = ys[y / 2 + 1] - ys[y / 2];
            ans += ex * ey;
        }
    }

    println!("{}", ans);
}
