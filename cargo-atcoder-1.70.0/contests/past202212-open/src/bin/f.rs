use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        x: Chars,
    };

    let x = x
        .into_iter()
        .filter(|c| c != &'.')
        .collect::<String>()
        .parse::<usize>()
        .unwrap();

    let sum = a + 2 * b + 3 * c + 4 * d;
    if sum * 1_000 <= x * n {
        println!("0");
        return;
    }

    let ans = ((1_000 * sum - x * n) + (x - 1000) - 1) / (x - 1000);

    println!("{}", ans);
}
