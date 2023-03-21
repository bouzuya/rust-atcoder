use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
    };
    let mut s = vec![vec![false; n]; m];
    for i in 0..m {
        input! {
            k_i: usize,
            s_i: [Usize1; k_i],
        }
        for s_ij in s_i {
            s[i][s_ij] = true;
        }
    }
    input! {
        p: [usize; m],
    }

    let mut count = 0_usize;
    for bits in 0..1 << n {
        let mut sw = vec![false; n];
        for i in 0..n {
            if (bits >> i) & 1 == 1 {
                sw[i] = true;
            }
        }
        let ok = s.iter().enumerate().all(|(i, s_i)| {
            let count = sw
                .iter()
                .copied()
                .zip(s_i.iter().copied())
                .filter(|&(a, b)| a && b)
                .count();
            count % 2 == p[i]
        });
        if ok {
            count += 1;
        }
    }

    let ans = count;
    println!("{}", ans);
}
