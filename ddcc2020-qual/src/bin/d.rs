use proconio::input;

fn main() {
    input! {
        m: usize,
        dc: [(usize, usize); m],
    };
    let d = dc.iter().copied().map(|(_, c_i)| c_i).sum::<usize>();
    let s = dc
        .iter()
        .copied()
        .map(|(d_i, c_i)| d_i * c_i)
        .sum::<usize>();
    let ans = d.saturating_sub(1) + s.saturating_sub(1) / 9;
    println!("{}", ans);
}
