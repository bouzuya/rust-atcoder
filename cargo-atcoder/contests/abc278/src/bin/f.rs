use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let fl = s
        .iter()
        .map(|s_i| {
            (
                ((*s_i.first().unwrap()) as u8 - b'a') as usize,
                ((*s_i.last().unwrap()) as u8 - b'a') as usize,
            )
        })
        .collect::<Vec<(usize, usize)>>();
    let mut dp = vec![vec![false; 26]; 1 << n];
    for used in (0..(1 << n) - 1).rev() {
        for c in 0..26 {
            dp[used][c] = fl
                .iter()
                .copied()
                .enumerate()
                .filter(|(i, _)| (used & (1_usize << i)) == 0)
                .any(|(i, (f, l))| c == f && !dp[used ^ (1 << i)][l]);
        }
    }

    let ans = dp[0].iter().copied().any(|b| b);
    println!("{}", if ans { "First" } else { "Second" });
}
