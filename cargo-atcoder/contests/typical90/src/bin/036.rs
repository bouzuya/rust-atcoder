use proconio::input;
use proconio::marker::Usize1;
use std::cmp;

fn main() {
    input! {
        large_n: usize,
        large_q: usize,
        xy: [(i64, i64); large_n],
        q: [Usize1; large_q],
    };
    let nx = |x: i64, y: i64| -> i64 { x + y };
    let ny = |x: i64, y: i64| -> i64 { x - y };
    let mut xy = xy
        .iter()
        .cloned()
        .enumerate()
        .collect::<Vec<(usize, (i64, i64))>>();
    let mut ps = vec![];
    xy.sort_by_key(|&(_, (x, y))| nx(x, y));
    ps.push(xy.first().unwrap().clone());
    ps.push(xy.last().unwrap().clone());
    xy.sort_by_key(|&(_, (x, y))| ny(x, y));
    ps.push(xy.first().unwrap().clone());
    ps.push(xy.last().unwrap().clone());
    xy.sort_by_key(|&(i, _)| i);
    for q_i in q {
        let (_, (x_i, y_i)) = xy[q_i];
        let ans = ps
            .iter()
            .map(|&(_, (x_j, y_j))| {
                cmp::max(
                    (nx(x_j, y_j) - nx(x_i, y_i)).abs(),
                    (ny(x_j, y_j) - ny(x_i, y_i)).abs(),
                )
            })
            .max()
            .unwrap();
        println!("{}", ans);
    }
}
