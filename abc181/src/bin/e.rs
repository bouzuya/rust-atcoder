use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut h: [i64; n],
        mut w: [i64; m],
    };
    h.sort();
    let mut e = vec![0_i64; n - 1];
    let mut o = vec![0_i64; n - 1];
    for i in 0..n - 1 {
        if i % 2 == 0 {
            e[i] = (h[i] - h[i + 1]).abs();
        } else {
            o[i] = (h[i] - h[i + 1]).abs();
        }
    }
    let mut s_e = vec![0_i64; n];
    let mut s_o = vec![0_i64; n];
    for i in 0..n - 1 {
        s_e[i + 1] += s_e[i] + e[i];
        s_o[i + 1] += s_o[i] + o[i];
    }

    let inf = 1 << 60;
    let mut min = inf;
    for w_i in w.iter().copied() {
        let j = h.lower_bound(&w_i);
        for dj in -1..=1 {
            let nj = j as i64 + dj;
            if (0..n as i64).contains(&nj) {
                let nj = nj as usize;
                let pos = nj;
                let left = s_e[pos];
                let curr = (w_i - h[pos]).abs();
                let right = s_o[n - 1] - s_o[pos];
                let x = left + curr + right;
                min = min.min(x);
            }
        }
    }
    println!("{}", min);
}
