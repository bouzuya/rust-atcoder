use std::cmp::Reverse;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut k: usize,
        x: usize,
        a: [usize; n],
    };

    let mut b = vec![];
    for a_i in a {
        let b_i = if a_i >= x {
            let count = a_i / x;
            if k >= count {
                k -= count;
                a_i - count * x
            } else {
                let y = a_i - k * x;
                k = 0;
                y
            }
        } else {
            a_i
        };
        if b_i == 0 {
            continue;
        }
        b.push(b_i);
    }

    // let mut c = b
    //     .iter()
    //     .copied()
    //     .map(|a_i| (a_i, a_i % x))
    //     .collect::<Vec<(usize, usize)>>();
    // c.sort_by_key(|&(b_i, m)| (Reverse(m), Reverse(b_i)));

    // let mut sum = 0_usize;
    // for (b_i, m) in c {
    //     let count = if m == 0 { b_i / x } else { (b_i + (x - 1)) / x };
    //     let c = count.min(k);
    //     k -= c;
    //     sum += b_i.saturating_sub(c * x);
    // }

    let mut c = b
        .iter()
        .copied()
        .map(|b_i| (b_i, x as i64 - b_i as i64))
        .collect::<Vec<(usize, i64)>>();
    c.sort_by_key(|&(_, v)| v);
    let mut sum = 0_usize;
    for (b_i, _) in c {
        if k > 0 {
            k -= 1;
            sum += b_i.saturating_sub(x);
        } else {
            sum += b_i
        }
    }

    let ans = sum;
    println!("{}", ans);
}
