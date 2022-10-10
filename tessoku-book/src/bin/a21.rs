use proconio::{input, marker::Usize1};

fn f(memo: &mut Vec<Vec<usize>>, pa: &[(usize, usize)], inf: usize, l: usize, r: usize) -> usize {
    if l >= r {
        return 0;
    }

    if memo[l][r] != inf {
        return memo[l][r];
    }
    let left = f(memo, pa, inf, l + 1, r)
        + if (l + 1..=r).contains(&pa[l].0) {
            pa[l].1
        } else {
            0
        };
    let right = f(memo, pa, inf, l, r - 1)
        + if (l..r).contains(&pa[r].0) {
            pa[r].1
        } else {
            0
        };
    let score = left.max(right);
    memo[l][r] = score;
    score
}

fn main() {
    input! {
        n: usize,
        pa: [(Usize1, usize); n],
    };
    let inf = 1_usize << 60;
    let mut memo = vec![vec![inf; n + 1]; n + 1];
    memo[0][0] = 0;
    let ans = f(&mut memo, &pa, inf, 0, n - 1);
    println!("{}", ans);
}
