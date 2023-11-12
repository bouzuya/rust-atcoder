use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize,
        q: usize,
        s: Chars,
        lr: [(Usize1, Usize1); q],
    };
    let ok = s
        .iter()
        .copied()
        .zip(s.iter().copied().skip(1))
        .map(|(s_i, s_j)| s_i == s_j)
        .collect::<Vec<bool>>();
    let c = std::iter::once(0_usize)
        .chain(ok.iter().scan(0, |acc, &ok_i| {
            *acc += if ok_i { 1 } else { 0 };
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    for (l_i, r_i) in lr {
        println!("{}", c[r_i] - c[l_i]);
    }
}
