use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        c: [[usize; 3]; 3],
    };

    // 0, 1, 2
    // 3, 4, 5
    // 6, 7, 8
    let c = {
        let mut c2 = vec![0_usize; 9];
        for i in 0..3 {
            for j in 0..3 {
                c2[i * 3 + j] = c[i][j];
            }
        }
        c2
    };

    let all = (1..=9).product::<usize>() as f64;
    let mut count = 0_f64;
    let mut ks = (0..9).collect::<Vec<usize>>();
    loop {
        let mut opened = vec![false; 9];
        let mut ok = true;
        for k in ks.iter().copied() {
            opened[k] = true;
            let oi = k / 3;
            let oj = k % 3;
            // |
            if (0..3).all(|i| opened[i * 3 + oj]) && {
                c[((oi + 1) % 3) * 3 + oj] == c[((oi + 2) % 3) * 3 + oj]
                    && c[((oi + 1) % 3) * 3 + oj] != c[oi * 3 + oj]
            } {
                ok = false;
            }
            // -
            if (0..3).all(|j| opened[oi * 3 + j]) && {
                c[oi * 3 + ((oj + 1) % 3)] == c[oi * 3 + ((oj + 2) % 3)]
                    && c[oi * 3 + ((oj + 1) % 3)] != c[oi * 3 + oj]
            } {
                ok = false;
            }
            // \
            if [0, 4, 8].into_iter().all(|x| opened[x])
                && [0, 4, 8].into_iter().any(|x| x == k)
                && {
                    match k {
                        0 => c[4] == c[8] && c[0] != c[4],
                        4 => c[0] == c[8] && c[4] != c[0],
                        8 => c[0] == c[4] && c[8] != c[0],
                        _ => unreachable!(),
                    }
                }
            {
                ok = false;
            }
            // \
            if [2, 4, 6].into_iter().all(|x| opened[x])
                && [2, 4, 6].into_iter().any(|x| x == k)
                && {
                    match k {
                        2 => c[4] == c[6] && c[2] != c[4],
                        4 => c[2] == c[6] && c[4] != c[2],
                        6 => c[2] == c[4] && c[6] != c[2],
                        _ => unreachable!(),
                    }
                }
            {
                ok = false;
            }
        }
        if ok {
            count += 1_f64;
        }
        if !ks.next_permutation() {
            break;
        }
    }
    let ans = count / all;
    println!("{}", ans);
}
