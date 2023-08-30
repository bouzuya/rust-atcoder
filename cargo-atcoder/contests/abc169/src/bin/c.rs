use proconio::{input, marker::Chars};

fn main() {
    input! {
        a: usize,
        b: Chars,
    };
    let b100 = b
        .iter()
        .copied()
        .filter(|b_i| b_i != &'.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();
    let ans = a * b100 / 100;
    println!("{}", ans);
}
