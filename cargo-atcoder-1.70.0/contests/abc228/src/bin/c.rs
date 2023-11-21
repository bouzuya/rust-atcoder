use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [[usize; 3]; n],
    };
    let mut is = p
        .into_iter()
        .map(|p_i| p_i.into_iter().sum::<usize>())
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    is.sort_by_key(|(_, s)| *s);

    let mut ans = vec![false; n];
    for (i, s) in is.iter().copied() {
        let j = is.lower_bound_by_key(&(s + 300 + 1), |(_, s)| *s);
        ans[i] = n + 1 - j <= k;
    }

    for a in ans {
        println!("{}", if a { "Yes" } else { "No" });
    }
}
