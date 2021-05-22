use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    };
    let ans = s
        .iter()
        .zip(t.iter())
        .flat_map(|(s_i, t_i)| vec![s_i, t_i])
        .collect::<String>();
    println!("{}", ans);
}
