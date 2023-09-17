use proconio::{input, marker::Chars};
use superslice::Ext;

fn main() {
    input! {
        m: usize,
        s: [Chars; 3],
    };
    let s = {
        let mut t = vec![vec![]; 3];
        for i in 0..3 {
            for _ in 0..3 {
                for j in 0..m {
                    t[i].push((s[i][j] as u8 - b'0') as usize);
                }
            }
        }
        t
    };

    let inf = 3 * m + 1;
    let mut min = inf;
    for d in 0..10 {
        let mut is = (0..3).collect::<Vec<usize>>();
        loop {
            let mut ok = true;
            let mut index = 0_usize;
            for i in is.iter().copied() {
                match s[i][index..].iter().copied().position(|c| c == d) {
                    None => {
                        ok = false;
                        break;
                    }
                    Some(j) => {
                        index = index + j + 1;
                    }
                }
            }
            if ok {
                min = min.min(index - 1);
            }
            if !is.next_permutation() {
                break;
            }
        }
    }

    let ans = if min == 3 * m + 1 { -1 } else { min as i64 };
    println!("{}", ans);
}
