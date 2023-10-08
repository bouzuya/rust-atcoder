use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };

    let mut players = s
        .iter()
        .enumerate()
        .map(|(i, s_i)| (i, s_i.iter().copied().filter(|s_ij| s_ij == &'o').count()))
        .collect::<Vec<(usize, usize)>>();
    players.sort_by_key(|&(i, win_count)| (Reverse(win_count), i));
    for (i, _) in players {
        println!("{}", i + 1);
    }
}
