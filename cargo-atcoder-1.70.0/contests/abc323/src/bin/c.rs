use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; m],
        s: [Chars; n],
    };

    let mut max = 0_usize;
    let mut score = vec![0_usize; n];
    for i in 0..n {
        score[i] += i + 1;
        for j in 0..m {
            if s[i][j] == 'o' {
                score[i] += a[j];
            }
        }

        max = max.max(score[i]);
    }

    let mut js = (0..m).map(|j| (j, a[j])).collect::<Vec<(usize, usize)>>();
    js.sort_by_key(|&(_, a)| Reverse(a));
    for i in 0..n {
        if score[i] == max {
            println!("0");
        } else {
            let mut cur = score[i];
            let mut count = 0_usize;
            for (j, a_j) in js.iter().copied() {
                if s[i][j] == 'o' {
                    continue;
                }
                cur += a_j;
                count += 1;
                if cur > max {
                    break;
                }
            }
            println!("{}", count);
        }
    }
}
