use proconio::input;

macro_rules! chmin {
    ($e: expr, $v: expr) => {
        if $v < $e {
            $e = $v;
        }
    };
}

fn f1(a: &Vec<Vec<i64>>, h: usize, w: usize, s: (usize, usize)) -> Vec<Vec<i64>> {
    let inf = 1_000_000_000_000_i64;
    let mut sp = vec![vec![inf; w]; h];
    let mut pq = std::collections::BinaryHeap::new();
    sp[s.0][s.1] = 0;
    pq.push(std::cmp::Reverse((0, (s.0, s.1))));
    while let Some(std::cmp::Reverse((c_u, (y, x)))) = pq.pop() {
        if c_u > sp[y][x] {
            continue;
        }
        let d = vec![-1, 0, 1];
        for (d_y, d_x) in d
            .iter()
            .flat_map(|d_y| d.iter().map(move |d_x| (d_y, d_x)))
            .filter(|&(d_y, d_x)| match (d_y, d_x) {
                (-1, -1) | (-1, 1) | (0, 0) | (1, -1) | (1, 1) => false,
                _ => true,
            })
        {
            let (ny, nx) = (y as i64 + d_y, x as i64 + d_x);
            if (0..h as i64).contains(&ny) && (0..w as i64).contains(&nx) {
                let (ny, nx) = (ny as usize, nx as usize);

                let c_v = a[ny][nx];
                let c = c_u + c_v;
                if c < sp[ny][nx] {
                    sp[ny][nx] = c;
                    pq.push(std::cmp::Reverse((c, (ny, nx))));
                }
            }
        }
    }
    sp
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
    };

    let inf = 1_000_000_000_000_i64;
    let mut ans = inf;
    for y in 0..h {
        for x in 0..w {
            let sp = f1(&a, h, w, (y, x));
            chmin!(
                ans,
                sp[h - 1][0] + sp[h - 1][w - 1] + sp[0][w - 1] + a[y][x]
            );
        }
    }
    println!("{}", ans);
}
