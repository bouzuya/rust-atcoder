use proconio::input;

fn main() {
    input! {
        n: usize,
        g: (i64, i64),
        xy: [(i64, i64); n],
    };
    let (ox, oy) = (250, 250);
    let inf = 1_000_000_000;
    let mut d = vec![vec![None; 500]; 500];
    for &(x, y) in xy.iter() {
        d[(y + oy) as usize][(x + ox) as usize] = Some(inf);
    }
    d[(0 + oy) as usize][(0 + ox) as usize] = Some(0);
    let mut deque = std::collections::VecDeque::new();
    deque.push_back((0, 0, 0));
    while let Some((cost, x, y)) = deque.pop_front() {
        let dir = vec![(1, 1), (0, 1), (-1, 1), (1, 0), (-1, 0), (0, -1)];
        for &(dx, dy) in dir.iter() {
            let nx = x + dx;
            let ny = y + dy;
            if (-201..=201).contains(&nx)
                && (-201..=201).contains(&ny)
                && d[(ny + oy) as usize][(nx + ox) as usize].is_none()
            {
                d[(ny + oy) as usize][(nx + ox) as usize] = Some(cost + 1);
                deque.push_back((cost + 1, nx, ny));
            }
        }
    }
    let ans = match d[(g.1 + oy) as usize][(g.0 + ox) as usize] {
        None => -1,
        Some(c) => c,
    };
    println!("{}", ans);
}
