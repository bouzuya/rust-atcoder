use proconio::input;

fn f1(
    a: &Vec<Vec<i64>>,
    h: usize,
    w: usize,
    s: (usize, usize),
) -> (Vec<Vec<i64>>, Vec<Vec<(usize, usize)>>) {
    let inf = 1_000_000_000_000_i64;
    let mut spt = vec![vec![(h, w); w]; h];
    let mut sp = vec![vec![inf; w]; h];
    let mut pq = std::collections::BinaryHeap::new();
    spt[s.0][s.1] = s;
    sp[s.0][s.1] = 0;
    pq.push(std::cmp::Reverse((0, (s.0, s.1, s.0, s.1))));
    while let Some(std::cmp::Reverse((c_u, (y, x, py, px)))) = pq.pop() {
        if c_u > sp[y][x] {
            continue;
        }
        spt[y][x] = (py, px);
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
                    pq.push(std::cmp::Reverse((c, (ny, nx, y, x))));
                }
            }
        }
    }
    (sp, spt)
}

fn f2(
    spt: &Vec<Vec<(usize, usize)>>,
    s: (usize, usize),
) -> std::collections::BTreeSet<(usize, usize)> {
    let mut c = s;
    let mut p = std::collections::BTreeSet::new();
    while c != spt[c.0][c.1] {
        p.insert(spt[c.0][c.1]);
        c = spt[c.0][c.1];
    }
    p
}

fn f3(
    a: &Vec<Vec<i64>>,
    p1: &std::collections::BTreeSet<(usize, usize)>,
    p2: &std::collections::BTreeSet<(usize, usize)>,
) -> i64 {
    let mut p_a = p1.clone();
    let mut p_b = p2.clone();
    p_a.append(&mut p_b);
    let mut res = 0;
    for &(y, x) in p_a.iter() {
        res += a[y][x];
    }
    res
}

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[i64; w]; h],
    };

    let (_, spt1) = f1(&a, h, w, (h - 1, 0));
    let (_, spt2) = f1(&a, h, w, (h - 1, w - 1));

    let p1 = f2(&spt1, (h - 1, w - 1));
    let p2 = f2(&spt1, (0, w - 1));
    let p3 = f2(&spt2, (0, w - 1));

    let res1 = f3(&a, &p1, &p3);
    let res2 = f3(&a, &p2, &p3);

    let ans = std::cmp::min(res1, res2);
    println!("{}", ans);
}
