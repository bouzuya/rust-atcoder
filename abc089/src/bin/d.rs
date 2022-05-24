use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        d: usize,
        a: [[usize; w]; h],
        q: usize,
        lr: [(usize, usize); q],
    };

    let mut pos = vec![(0, 0); h * w + 1];
    for i in 0..h {
        for j in 0..w {
            pos[a[i][j]] = (i, j);
        }
    }

    let mut mp = vec![0; h * w + 1];
    for i in d..=h * w {
        let (x_l, y_l) = pos[i - d];
        let (x_r, y_r) = pos[i];
        let f = |a: usize, b: usize| -> usize {
            if a > b {
                a - b
            } else {
                b - a
            }
        };
        let d_d = f(x_r, x_l) + f(y_r, y_l);
        mp[i] += mp[i - d] + d_d;
    }

    let mut cumsum = vec![0_usize; h * w + 1];
    for i in 1..=d {
        for j in (i..=h * w).step_by(d) {
            cumsum[j] += mp[j];
        }
    }

    for (l, r) in lr {
        println!("{}", cumsum[r] - cumsum[l]);
    }
}
