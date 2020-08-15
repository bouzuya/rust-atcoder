use proconio::input;
use proconio::marker::Usize1;

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

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [Usize1; n],
        c: [i64; n],
    };

    let mut vs = vec![];
    let mut b = vec![false; n];
    for i in 0..n {
        if !b[i] {
            let mut v = vec![];
            let mut c = i;
            while !b[c] {
                b[c] = true;
                v.push(c);
                c = p[c];
            }
            vs.push(v);
        }
    }

    let mut ans = c[vs[0][0]];
    for v in vs.iter() {
        let sum_v = v.iter().map(|&v_i| c[v_i]).sum::<i64>();
        let n_v = v.len();
        let mut a = c[v[0]];
        for i in 0..n_v {
            let mut s = 0_i64;
            for j in i..std::cmp::min(2 * n_v, i + k) {
                s += c[v[j % n_v]];
                chmax!(a, s);
            }
        }
        if k > n_v {
            let x = k / n_v;
            let k = k % n_v;
            for i in 0..n_v {
                let mut s = (x as i64 - 1) * sum_v;
                chmax!(a, s);
                for j in i..i + n_v + k {
                    s += c[v[j % n_v]];
                    chmax!(a, s);
                }
            }
        }
        chmax!(ans, a);
    }
    println!("{}", ans);
}
