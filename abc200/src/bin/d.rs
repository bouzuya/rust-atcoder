use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut b = vec![vec![]; 200];
    for bits in 0..1 << cmp::min(8, n) {
        let is = (0..n)
            .filter(|i| (bits >> i) & 1 == 1)
            .collect::<Vec<usize>>();
        if is.is_empty() {
            continue;
        }
        let x = is.iter().map(|&i| a[i] % 200).sum::<usize>() % 200;
        b[x].push(is);
        if b[x].len() >= 2 {
            println!("Yes");
            println!(
                "{} {}",
                b[x][0].len(),
                b[x][0]
                    .iter()
                    .map(|&i| format!("{}", i + 1))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            println!(
                "{} {}",
                b[x][1].len(),
                b[x][1]
                    .iter()
                    .map(|&i| format!("{}", i + 1))
                    .collect::<Vec<String>>()
                    .join(" ")
            );
            return;
        }
    }
    println!("No");
}
