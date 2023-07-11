use proconio::{input, marker::Chars};

fn main() {
    input! {
        _n: usize,
        s: Chars,
        t: Chars,
    };
    let s = s
        .iter()
        .enumerate()
        .filter(|(_, &s_i)| s_i == '0')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    let t = t
        .iter()
        .enumerate()
        .filter(|(_, &t_i)| t_i == '0')
        .map(|(i, _)| i)
        .collect::<Vec<usize>>();
    if s.len() != t.len() {
        println!("-1");
        return;
    }
    let mut count = 0;
    for (s_i, t_i) in s.iter().copied().zip(t.iter().copied()) {
        if s_i != t_i {
            count += 1;
        }
    }
    let ans = count;
    println!("{}", ans);
}
