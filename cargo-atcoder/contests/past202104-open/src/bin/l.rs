use ordered_float::NotNan;
use proconio::input;

fn prim(e: &[Vec<(usize, NotNan<f64>)>], s: usize) -> NotNan<f64> {
    let n = e.len();
    let mut used = vec![false; n];
    let mut d = NotNan::new(0_f64).unwrap();
    let mut pq = std::collections::BinaryHeap::new();
    pq.push(std::cmp::Reverse((NotNan::new(0_f64).unwrap(), s)));
    while let Some(std::cmp::Reverse((c_u, u))) = pq.pop() {
        if used[u] {
            continue;
        }
        used[u] = true;
        d += c_u;
        for &(v, c_v) in e[u].iter() {
            if !used[v] {
                pq.push(std::cmp::Reverse((c_v, v)));
            }
        }
    }
    d
}

fn distance((x_i, y_i, r_i): (f64, f64, f64), (x_j, y_j, r_j): (f64, f64, f64)) -> f64 {
    let d = ((x_i - x_j).powf(2_f64) + (y_i - y_j).powf(2_f64)).sqrt();
    if d > (r_i + r_j) {
        d - (r_i + r_j)
    } else if d <= (r_i + r_j) && (r_i - r_j).abs() > d {
        (r_i - r_j).abs() - d
    } else {
        0_f64
    }
}

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [(i64, i64); n],
        c: [(i64, i64, i64); m],
    };
    let inf = NotNan::new(1_000_000_000_000_f64).unwrap();

    let mut res = inf;
    for bits in 0..1 << m {
        let s = p
            .iter()
            .copied()
            .map(|(px_i, py_i)| (px_i as f64, py_i as f64, 0_f64))
            .chain(
                c.iter()
                    .enumerate()
                    .filter(|&(j, _)| (bits >> j) & 1 == 1)
                    .map(|(_, &(cx_i, cy_i, r_i))| (cx_i as f64, cy_i as f64, r_i as f64)),
            )
            .collect::<Vec<(f64, f64, f64)>>();
        let mut edges = vec![vec![]; s.len()];
        for i in 0..s.len() {
            for j in i + 1..s.len() {
                let w = distance(s[i], s[j]);
                edges[i].push((j, NotNan::new(w).unwrap()));
                edges[j].push((i, NotNan::new(w).unwrap()));
            }
        }

        let sum = prim(&edges, 0);
        res = res.min(sum);
    }
    let ans = res;
    println!("{}", ans);
}
