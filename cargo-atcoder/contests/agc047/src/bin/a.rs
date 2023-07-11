// <https://tiramistercp.hatenablog.com/entry/agc047-a>
use std::cmp;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    };
    let b = a
        .into_iter()
        .map(|a_i| {
            let mut s = vec![];
            let mut pos = None;
            for (j, c) in a_i.into_iter().enumerate() {
                match c {
                    '.' => {
                        pos = Some(j);
                    }
                    d if c.is_ascii_digit() => {
                        s.push(d);
                    }
                    _ => unreachable!(),
                }
            }
            s.iter().collect::<String>().parse::<usize>().unwrap()
                * 10_usize.pow((9 - (s.len() - pos.unwrap_or_else(|| s.len()))) as u32)
        })
        .collect::<Vec<usize>>();

    let mut count = vec![vec![0_usize; 18 + 1]; 18 + 1];
    for b_i in b {
        let mut m = b_i;
        let mut c2 = 0;
        while m % 2 == 0 {
            m /= 2;
            c2 += 1;
        }
        let mut c5 = 0;
        while m % 5 == 0 {
            m /= 5;
            c5 += 1;
        }
        count[cmp::min(c2, 18)][cmp::min(c5, 18)] += 1;
    }

    let mut sum = 0_usize;
    for i2 in 0..=18 {
        for j2 in 0..=18 {
            for i5 in 0..=18 {
                for j5 in 0..=18 {
                    if i2 + j2 >= 18 && i5 + j5 >= 18 {
                        sum += count[i2][i5]
                            * count[j2][j5].saturating_sub(if i2 == j2 && i5 == j5 {
                                1
                            } else {
                                0
                            });
                    }
                }
            }
        }
    }
    let ans = sum / 2;
    println!("{}", ans);
}
