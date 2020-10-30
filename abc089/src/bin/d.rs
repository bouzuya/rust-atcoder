use proconio::input;
use proconio::marker::Usize1;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        a: [[Usize1; w]; h],
        q: usize,
        lr: [(Usize1, Usize1); q],
    };
    let mut p = vec![(0, 0); h * w];
    for i in 0..h {
        for j in 0..w {
            p[a[i][j]] = (i, j);
        }
    }
    let mut c = vec![0; h * w];
    for i in 0..h * w - d {
        let (l_x, l_y) = p[i];
        let (r_x, r_y) = p[i + d];
        c[i + d] = c[i] + (r_x as i64 - l_x as i64).abs() + (r_y as i64 - l_y as i64).abs();
    }
    for (l_i, r_i) in lr {
        let ans = c[r_i] - c[l_i];
        println!("{}", ans);
    }
}
