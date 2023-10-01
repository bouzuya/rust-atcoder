use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        m: usize,
        s: [Chars; 3],
    };
    let s = {
        vec![
            s[0].iter()
                .copied()
                .chain(s[0].iter().copied())
                .chain(s[0].iter().copied())
                .collect::<Vec<char>>(),
            s[1].iter()
                .copied()
                .chain(s[1].iter().copied())
                .chain(s[1].iter().copied())
                .collect::<Vec<char>>(),
            s[2].iter()
                .copied()
                .chain(s[2].iter().copied())
                .chain(s[2].iter().copied())
                .collect::<Vec<char>>(),
        ]
    };

    let inf = 3 * m + 1;
    let mut min = inf;
    let mut is = (0..3).collect::<Vec<usize>>();
    loop {
        for (i, c) in s[is[0]].iter().copied().enumerate() {
            if let Some(j) = s[is[1]][i + 1..]
                .iter()
                .position(|&c_i| c_i == c)
                .map(|j| i + 1 + j)
            {
                if let Some(k) = s[is[2]][j + 1..]
                    .iter()
                    .position(|&c_i| c_i == c)
                    .map(|k| j + 1 + k)
                {
                    min = min.min(k);
                }
            }
        }
        if !is.next_permutation() {
            break;
        }
    }
    if min == inf {
        println!("-1");
    } else {
        println!("{}", min);
    }
}
