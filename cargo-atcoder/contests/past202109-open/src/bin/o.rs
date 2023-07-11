// <https://atcoder.jp/contests/past202109-open/editorial/2358>
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        p: [Usize1; 1 << n],
        wl: [(Usize1, Usize1); m],
    };

    let mut q = vec![0; 1 << 17];
    for (i, p_i) in p.into_iter().enumerate() {
        q[p_i] = (1 << n) + i;
    }

    let mut a = vec![None; 1 << n];
    let mut ok = true;
    for i in 0..m {
        let mut x = q[wl[i].0];
        let mut y = q[wl[i].1];
        while (x / 2) != (y / 2) {
            match a[x / 2] {
                Some(value) => {
                    if value != x % 2 {
                        ok = false;
                    }
                }
                None => {
                    a[x / 2] = Some(x % 2);
                }
            }

            match a[y / 2] {
                Some(value) => {
                    if value != y % 2 {
                        ok = false;
                    }
                }
                None => {
                    a[y / 2] = Some(y % 2);
                }
            }

            x /= 2;
            y /= 2;
        }

        match a[x / 2] {
            Some(value) => {
                if value != x % 2 {
                    ok = false;
                }
            }
            None => {
                a[x / 2] = Some(x % 2);
            }
        }
    }

    if ok {
        let ans = a
            .into_iter()
            .skip(1)
            .take((1 << n) - 1)
            .filter(|a_i| a_i.is_none())
            .fold(2, |acc, _| (acc * 2) % 998_244_353);
        println!("{}", ans)
    } else {
        println!("{}", 0);
    }
}
