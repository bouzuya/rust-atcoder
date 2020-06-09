use proconio::input;

macro_rules! chmax {
    ($e: expr, $v: expr) => {
        if $v > $e {
            $e = $v;
        }
    };
}

fn main() {
    input! {
        n: usize, // n <= 50
        d: [[i64; n]; n], // d[i][j] <= 100
        q: usize, // q <= 2500
        p: [usize; q], // p[i] <= 2500
    };

    let mut cs = vec![vec![0; n + 1]; n + 1];
    for y_d in 0..n {
        for x_d in 0..n {
            let d_ij = d[y_d][x_d];
            for y_cs in y_d..n {
                for x_cs in x_d..n {
                    cs[y_cs + 1][x_cs + 1] += d_ij;
                }
            }
        }
    }

    let mut max_v = 0;
    let mut ans = vec![0; n * n + 1];
    for p in 1..=n * n {
        for h in 1..=std::cmp::min(p, n) {
            let w = p / h;
            if w * h != p {
                continue;
            }

            for y in h..=n {
                for x in w..=n {
                    let v = cs[y][x] - cs[y][x - w] - cs[y - h][x] + cs[y - h][x - w];
                    chmax!(max_v, v);
                }
            }
        }
        chmax!(ans[p], max_v);
    }

    for &p_i in p.iter() {
        println!("{}", ans[p_i]);
    }
}
