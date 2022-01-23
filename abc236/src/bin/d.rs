// TLE
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
    };
    let a = {
        let mut a = vec![];
        for i in 0..2 * n {
            input! {
                a_i: [usize; 2 * n - i - 1]
            }
            a.push(a_i);
        }
        let mut a2 = vec![vec![0; 2 * n]; 2 * n];
        for i in 0..2 * n {
            for j in 0..2 * n - i - 1 {
                a2[i][i + 1 + j] = a[i][j];
                a2[i + 1 + j][i] = a[i][j];
            }
        }
        a2
    };

    // for i in 0..2 * n {
    //     for j in 0..2 * n {
    //         print!("{} ", a[i][j]);
    //     }
    //     println!();
    // }

    let mut ans = 0;
    for bits in 0..1_usize << 2 * n {
        if bits.count_ones() as usize != n {
            continue;
        }

        let mut is = vec![];
        let mut js = vec![];
        for i in 0..2 * n {
            if (bits >> i) & 1 == 1 {
                is.push(i);
            } else {
                js.push(i);
            }
        }

        let mut max = 0;

        let mut pairs = (0..n).collect::<Vec<usize>>();
        loop {
            let mut b = 0;
            for i in 0..n {
                b ^= a[is[i]][js[pairs[i]]];
            }
            max = max.max(b);
            if !pairs.next_permutation() {
                break;
            }
        }

        ans = ans.max(max);
    }
    println!("{}", ans);
}
