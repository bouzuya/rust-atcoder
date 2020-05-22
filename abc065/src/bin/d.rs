use proconio::input;

fn f(n: usize, e: &Vec<Vec<(i64, usize)>>, s: usize) -> i64 {
    let inf = 1_000_000_000_000_000_i64;
    let mut d = vec![inf; n];
    let mut t = vec![false; n];
    let mut pq = std::collections::BinaryHeap::new();
    let mut res = 0;
    d[s] = 0;
    pq.push(std::cmp::Reverse((0, s)));
    while let Some(std::cmp::Reverse((c_u, u))) = pq.pop() {
        if t[u] {
            continue;
        }
        t[u] = true;
        res += c_u;
        for &(c_v, v) in e[u].iter() {
            if t[v] {
                continue;
            }
            pq.push(std::cmp::Reverse((c_v, v)));
        }
    }
    res
}

fn main() {
    input! {
        n: usize,
        xy: [(i64, i64); n],
    };
    let mut e = vec![vec![]; n];
    let mut x = xy
        .iter()
        .enumerate()
        .map(|(i, &(x, _))| (x, i))
        .collect::<Vec<(i64, usize)>>();
    x.sort();
    for w in x.windows(2) {
        match w {
            &[(x_i, i), (x_j, j)] => {
                e[i].push(((x_i - x_j).abs(), j));
                e[j].push(((x_i - x_j).abs(), i));
            }
            _ => unreachable!(),
        }
    }
    let mut y = xy
        .iter()
        .enumerate()
        .map(|(i, &(_, y))| (y, i))
        .collect::<Vec<(i64, usize)>>();
    y.sort();
    for w in y.windows(2) {
        match w {
            &[(y_i, i), (y_j, j)] => {
                e[i].push(((y_i - y_j).abs(), j));
                e[j].push(((y_i - y_j).abs(), i));
            }
            _ => unreachable!(),
        }
    }

    let ans = f(n, &e, 0);
    println!("{}", ans);
}
