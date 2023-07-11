use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ct: [(usize, usize); n],
    };
    let ans = ct
        .iter()
        .copied()
        .filter(|&(_, t_i)| t_i <= t)
        .map(|(c_i, _)| c_i)
        .min()
        .map(|v| format!("{}", v))
        .unwrap_or_else(|| "TLE".to_string());
    println!("{}", ans);
}
