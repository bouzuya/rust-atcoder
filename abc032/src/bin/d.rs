use proconio::input;

macro_rules! chmax {
    ($max_v: expr, $v: expr) => {
        if $v > $max_v {
            $max_v = $v;
            true
        } else {
            false
        }
    };
}

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

fn f(vw: &[(usize, usize)], w: usize) -> Vec<(usize, usize)> {
    let n = vw.len();
    let mut map = std::collections::BTreeMap::new();
    for bits in 0..1 << n {
        let mut sum_w = 0;
        let mut sum_v = 0;
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                let (v_i, w_i) = vw[i];
                sum_w += w_i;
                sum_v += v_i;
            }
        }
        if sum_w > w {
            continue;
        }
        let entry = map.entry(sum_w).or_insert(0);
        if *entry < sum_v {
            *entry = sum_v;
        }
    }
    let mut b = vec![true; map.len()];
    for (i, (w_i, v_i)) in map.iter().enumerate() {
        if !b[i] {
            continue;
        }
        for (j, (w_j, v_j)) in map.iter().enumerate() {
            if !b[j] {
                continue;
            }
            if w_j > w_i && v_j <= v_i {
                b[j] = false;
            }
        }
    }
    map.iter()
        .zip(b)
        .filter(|&(_, b_i)| b_i)
        .map(|((&w_i, &v_i), _)| (w_i, v_i))
        .collect::<Vec<_>>()
}

fn main() {
    input! {
        n: usize,
        w: usize,
        vw: [(usize, usize); n],
    };
    if n <= 30 {
        let wv1 = f(&vw[0..n / 2], w);
        let wv2 = f(&vw[n / 2..n], w);
        let mut ans = 0;
        for &(w_i, v_i) in wv1.iter() {
            // binary search
            let mut l = 0;
            let mut r = wv2.len();
            while r - l > 1 {
                let m = l + (r - l) / 2;
                let (w_m, _) = wv2[m];
                if w_m + w_i <= w {
                    l = m;
                } else {
                    r = m;
                }
            }
            let (w_j, v_j) = wv2[l];
            if w_i + w_j <= w {
                chmax!(ans, v_i + v_j);
            }
        }
        println!("{}", ans);
    } else if vw.iter().all(|&(_, w_i)| w_i <= 1_000) {
        let mut dp = vec![vec![0; w + 1]; n + 1];
        for (i, &(v_i, w_i)) in vw.iter().enumerate() {
            for j in 0..=w {
                chmax!(dp[i + 1][j], dp[i][j]);
                if j + w_i <= w {
                    chmax!(dp[i + 1][j + w_i], dp[i][j] + v_i);
                }
            }
        }
        let ans = dp[n].iter().max().unwrap();
        println!("{}", ans);
    } else if vw.iter().all(|&(v_i, _)| v_i <= 1_000) {
        let inf = 1_000_000_000_000;
        let max_v = vw.iter().map(|&(v_i, _)| v_i).max().unwrap();
        let mut dp = vec![vec![inf; max_v * n + 1]; n + 1];
        dp[0][0] = 0;
        for (i, &(v_i, w_i)) in vw.iter().enumerate() {
            for j in 0..=max_v * n {
                chmin!(dp[i + 1][j], dp[i][j]);
                if j + v_i <= max_v * n {
                    chmin!(dp[i + 1][j + v_i], dp[i][j] + w_i);
                }
            }
        }
        let ans = dp[n]
            .iter()
            .enumerate()
            .filter(|&(_, &w_i)| w_i <= w)
            .map(|(v_i, _)| v_i)
            .max()
            .unwrap();
        println!("{}", ans);
    }
}
