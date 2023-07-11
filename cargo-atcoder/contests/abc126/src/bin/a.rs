use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        _n: usize,
        k: Usize1,
        s: Chars,
    };
    let ans = s
        .iter()
        .copied()
        .enumerate()
        .map(|(i, s_i)| {
            if i == k {
                s_i.to_lowercase().next().unwrap()
            } else {
                s_i
            }
        })
        .collect::<String>();
    println!("{}", ans);
}
