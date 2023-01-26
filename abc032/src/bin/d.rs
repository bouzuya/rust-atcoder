use std::collections::BTreeMap;

use proconio::input;
use superslice::Ext;

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

fn f(w: usize, vw: &[(usize, usize)]) -> Vec<(usize, usize)> {
    let n = vw.len();
    let mut map = BTreeMap::new();
    for bits in 0..1 << n {
        let mut sum_v = 0_usize;
        let mut sum_w = 0_usize;
        for i in 0..n {
            if ((bits >> i) & 1) == 1 {
                let (v_i, w_i) = vw[i];
                sum_v += v_i;
                sum_w += w_i;
            }
        }
        if sum_w <= w {
            let entry = map.entry(sum_w).or_insert(sum_v);
            *entry = (*entry).max(sum_v);
        }
    }
    let mut res = vec![];
    let mut maxv = 0;
    for (k, v) in map {
        maxv = v.max(maxv);
        res.push((k, maxv));
    }
    res
}

fn main() {
    input! {
        n: usize,
        w: usize,
        vw: [(usize, usize); n],
    };
    let ans = if n <= 30 {
        let wv1 = f(w, &vw[0..n / 2]);
        let wv2 = f(w, &vw[n / 2..n]);
        let mut ans = 0_usize;
        for (w1, v1) in wv1 {
            let i = wv2
                .lower_bound_by_key(&(w + 1 - w1), |(w2, _)| *w2)
                .saturating_sub(1);
            if i < wv2.len() {
                let (w2, v2) = wv2[i];
                if w1 + w2 <= w {
                    ans = ans.max(v1 + v2);
                }
            }
        }
        ans
    } else if vw.iter().copied().all(|(_, w_i)| w_i <= 1_000) {
        let mut dp = vec![0_usize; w + 1];
        for (v_i, w_i) in vw {
            let mut next = vec![0_usize; w + 1];
            for j in 0..=w {
                chmax!(next[j], dp[j]);
                if j + w_i <= w {
                    chmax!(next[j + w_i], dp[j] + v_i);
                }
            }
            dp = next;
        }
        *dp.iter().max().unwrap()
    } else if vw.iter().copied().all(|(v, _)| v <= 1_000) {
        let inf = 1 << 60;
        let mut dp = vec![inf; 1_000 * n + 1];
        dp[0] = 0_usize;
        for (v_i, w_i) in vw {
            let mut next = vec![inf; 1_000 * n + 1];
            next[0] = 0_usize;
            for j in 0..=1_000 * n {
                chmin!(next[j], dp[j]);
                if j + v_i <= 1_000 * n {
                    chmin!(next[j + v_i], dp[j] + w_i);
                }
            }
            dp = next;
        }
        dp.into_iter()
            .enumerate()
            .filter(|&(_, w_i)| w_i <= w)
            .map(|(v, _)| v)
            .max()
            .unwrap_or(0)
    } else {
        unreachable!()
    };

    println!("{}", ans);
}
