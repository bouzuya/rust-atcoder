use proconio::input;
use proconio::marker::Usize1;

fn dfs(
    ais: &mut Vec<usize>,
    n: usize,
    m: usize,
    abcds: &Vec<(usize, usize, usize, usize)>,
) -> usize {
    if ais.len() == n {
        return abcds
            .iter()
            .map(|&(a, b, c, d)| if ais[b] - ais[a] == c { d } else { 0 })
            .sum();
    }

    let &ai = ais.last().unwrap_or(&1);
    let mut res = 0;
    for aj in ai..=m {
        ais.push(aj);
        res = std::cmp::max(res, dfs(ais, n, m, abcds));
        ais.pop();
    }
    res
}

fn main() {
    input! {
        n: usize,
        m: usize,
        q: usize,
        abcds: [(Usize1, Usize1, usize, usize); q],
    };

    let mut ais = vec![];
    let ans = dfs(&mut ais, n, m, &abcds);
    println!("{}", ans);
}
