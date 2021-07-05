use proconio::{input, marker::Usize1};

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
        abc: [(Usize1, Usize1, u64); m],
    };

    let inf = 1_000_000_000_000_000;
    let mut d = vec![vec![inf; n]; n];
    for (i, d_i) in d.iter_mut().enumerate() {
        d_i[i] = 0;
    }
    for (a_i, b_i, c_i) in abc {
        d[a_i][b_i] = c_i;
    }
    let mut sum = 0_u64;
    for k in 0..n {
        for u in 0..n {
            for v in 0..n {
                chmin!(d[u][v], d[u][k] + d[k][v]);
                sum += if d[u][v] == inf { 0 } else { d[u][v] };
            }
        }
    }
    let ans = sum;
    println!("{}", ans);
}
