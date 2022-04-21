use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        x: usize,
        y: usize,
        a: usize,
        b: usize,
        c: usize,
        p: [usize; a],
        q: [usize; b],
        r: [usize; c],
    };
    let mut pqr = p
        .into_iter()
        .map(|p_i| (0, p_i))
        .chain(q.into_iter().map(|q_i| (1, q_i)))
        .chain(r.into_iter().map(|r_i| (2, r_i)))
        .collect::<Vec<(usize, usize)>>();
    pqr.sort_by_key(|&(t, v)| (Reverse(v), t));

    let mut sum = 0_usize;
    let mut count_a = 0_usize;
    let mut count_b = 0_usize;
    let mut count_c = 0_usize;
    for (t, v) in pqr {
        if count_a + count_b + count_c >= x + y {
            break;
        }
        match t {
            0 => {
                if count_a >= x {
                    continue;
                }
                count_a += 1;
                sum += v;
            }
            1 => {
                if count_b >= y {
                    continue;
                }
                count_b += 1;
                sum += v;
            }
            2 => {
                count_c += 1;
                sum += v;
            }
            _ => unreachable!(),
        }
    }

    let ans = sum;
    println!("{}", ans);
}
