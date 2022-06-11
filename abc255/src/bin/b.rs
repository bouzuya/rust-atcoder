use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [Usize1; k],
        xy: [(i64, i64); n],
    };

    let mut max_d = 0_f64;
    for (x_i, y_i) in xy.iter().copied() {
        let mut min_d = 10_000_000_000_f64;
        for a_j in a.iter().copied() {
            let (x_j, y_j) = xy[a_j];
            let d = (((x_i - x_j).pow(2) + (y_i - y_j).pow(2)) as f64).sqrt();
            min_d = min_d.min(d);
        }
        max_d = max_d.max(min_d);
    }

    let ans = max_d;
    println!("{}", ans);
}
