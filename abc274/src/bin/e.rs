use proconio::input;

macro_rules! chmin {
    ($min_v: expr, $v: expr) => {
        if $v < $min_v {
            $min_v = $v;
            true
        } else {
            false
        }
    };
}

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(f64, f64); n],
        pq: [(f64, f64); m],
    };
    let n = n + 1;
    let xy = {
        let mut xy2 = vec![(0_f64, 0_f64)];
        xy2.extend(xy);
        xy2.extend(pq);
        xy2
    };
    let mut dist = vec![vec![0_f64; n + m]; n + m];
    for i in 0..n + m {
        for j in i + 1..n + m {
            let (x_i, y_i) = xy[i];
            let (x_j, y_j) = xy[j];
            let d = ((x_i - x_j).powf(2_f64) + (y_i - y_j).powf(2_f64)).sqrt();
            dist[i][j] = d;
            dist[j][i] = d;
        }
    }

    let inf = 1e18;
    let mut ans = inf;
    for bits in 0_usize..1 << m {
        let mut is = vec![];
        let mut mask = 0_usize;
        for i in 0..m {
            if ((bits >> i) & 1) == 1 {
                is.push(n + i);
                mask <<= 1;
                mask |= 1;
            }
        }
        mask <<= n;
        let l = bits.count_ones() as usize;

        let mut dp = vec![vec![inf; n + l]; 1 << (n + l)];
        dp[1][0] = 0_f64;
        for set in 0..1 << (n + l) {
            let speed = (1 << (set & mask).count_ones()) as f64;
            for i in 0..n + l {
                if (dp[set][i] - inf).abs() < std::f64::EPSILON {
                    continue;
                }
                for j in 0..n + l {
                    if i == j {
                        continue;
                    }
                    let ii = if i < n { i } else { is[i - n] };
                    let ij = if j < n { j } else { is[j - n] };
                    let time = dist[ii][ij] / speed;
                    chmin!(dp[set | (1 << j)][j], dp[set][i] + time);
                }
            }
        }

        let mut a = inf;
        let set = (1 << (n + l)) - 1;
        let speed = (1 << (set & mask).count_ones()) as f64;
        for (i, d) in dp[set].iter().copied().enumerate() {
            let ii = if i < n { i } else { is[i - n] };
            let time = dist[ii][0] / speed;
            chmin!(a, d + time);
        }
        chmin!(ans, a);
    }
    println!("{}", ans);
}
