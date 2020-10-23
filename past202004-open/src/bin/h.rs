use proconio::input;
use proconio::marker::Chars;

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
        a: [Chars; n],
    };
    let mut ps = vec![vec![]; 9];
    let mut s = (n as i64, m as i64);
    let mut g = (n as i64, m as i64);
    for r in 0..n {
        for c in 0..m {
            match a[r][c] {
                'S' => s = (r as i64, c as i64),
                'G' => g = (r as i64, c as i64),
                d => ps[(d as u8 - '1' as u8) as usize].push((r as i64, c as i64)),
            }
        }
    }
    let inf = 1_000_000_000;
    let mut prev = vec![(s.0, s.1, 0)];
    for i in 0..9 {
        let mut next = ps[i]
            .iter()
            .map(|&(n_r, n_c)| (n_r, n_c, inf))
            .collect::<Vec<(i64, i64, i64)>>();
        for (p_r, p_c, p_w) in prev.iter() {
            if ps[i].is_empty() {
                println!("-1");
                return;
            }
            for (i, &(n_r, n_c)) in ps[i].iter().enumerate() {
                chmin!(next[i].2, p_w + (p_r - n_r).abs() + (p_c - n_c).abs());
            }
        }
        prev = next;
    }
    let mut ans = inf;
    for (p_r, p_c, p_w) in prev.iter() {
        chmin!(ans, p_w + (p_r - g.0).abs() + (p_c - g.1).abs());
    }
    println!("{}", ans);
}
