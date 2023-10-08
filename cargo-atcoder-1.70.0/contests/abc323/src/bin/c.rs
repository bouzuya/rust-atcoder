use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; m],
        s: [Chars; n],
    };

    let score = s
        .iter()
        .enumerate()
        .map(|(i, s_i)| {
            i + 1
                + s_i
                    .iter()
                    .copied()
                    .zip(a.iter().copied())
                    .filter(|(s_ij, _)| s_ij == &'o')
                    .map(|(_, a_i)| a_i)
                    .sum::<usize>()
        })
        .collect::<Vec<usize>>();
    let max = score.iter().copied().max().unwrap();

    let mut ordered_a = a
        .iter()
        .copied()
        .enumerate()
        .collect::<Vec<(usize, usize)>>();
    ordered_a.sort_by_key(|&(_, a)| Reverse(a));

    let ans = score
        .iter()
        .copied()
        .enumerate()
        .map(|(i, score_i)| {
            std::iter::once(score_i)
                .chain(
                    ordered_a
                        .iter()
                        .copied()
                        .filter_map(|(j, a_j)| (s[i][j] == 'x').then_some(a_j)),
                )
                .scan(0, |acc, x| {
                    *acc += x;
                    Some(*acc)
                })
                .take_while(|sum| sum < &max)
                .count()
        })
        .collect::<Vec<usize>>();

    for a in ans {
        println!("{}", a);
    }
}
