use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        m: usize,
        xyz: [(i64, i64, i64); n],
    };
    let mut max_sum = 0;
    for bits in 0..1 << 3 {
        let ps = (0..n).map(|i| (bits >> i) & 1 == 1).collect::<Vec<bool>>();
        let mut vs = vec![];
        for &(x_i, y_i, z_i) in xyz.iter() {
            let v = if ps[0] { x_i } else { -x_i }
                + if ps[1] { y_i } else { -y_i }
                + if ps[2] { z_i } else { -z_i };
            vs.push(v);
        }
        vs.sort_by_key(|x| -x);
        let sum = vs[0..m].iter().sum::<i64>();
        max_sum = max(max_sum, sum);
    }
    let ans = max_sum;
    println!("{}", ans);
}
