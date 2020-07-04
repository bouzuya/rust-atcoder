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
        m: usize,
        xy: [(Usize1, Usize1); m],
    };
    let mut e = vec![vec![]; n];
    let mut d = vec![0; n]; // 入次数 (indegree)
    for &(x_i, y_i) in xy.iter() {
        e[x_i].push(y_i);
        d[y_i] += 1;
    }

    let mut q = d
        .iter()
        .enumerate()
        .filter(|&(_, &d_i)| d_i == 0)
        .map(|(v_i, _)| v_i)
        .collect::<std::collections::VecDeque<usize>>();

    let mut dp = vec![0; n];
    while let Some(u) = q.pop_front() {
        for &v in e[u].iter() {
            chmax!(dp[v], dp[u] + 1);
            d[v] -= 1;
            if d[v] == 0 {
                q.push_back(v);
            }
        }
    }

    let ans = dp.iter().max().unwrap();
    println!("{}", ans);
}
