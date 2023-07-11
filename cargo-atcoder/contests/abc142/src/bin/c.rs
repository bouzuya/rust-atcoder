use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [Usize1; n],
    };
    let mut b = a
        .iter()
        .enumerate()
        .map(|(i, &a_i)| (i, a_i))
        .collect::<Vec<(usize, usize)>>();
    b.sort_by_key(|&(_, a_i)| a_i);
    let ans = b
        .iter()
        .map(|(i, _)| format!("{}", i + 1))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", ans);
}
