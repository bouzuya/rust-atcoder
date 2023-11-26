use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };
    let mut x = vec![];
    for _ in 0..n {
        input! {
            c_i: usize,
            x_i: [Usize1; c_i],
        }
        let x_i = x_i.iter().copied().collect::<HashSet<usize>>();
        x.push(x_i);
    }
    input! {
        q: usize,
    }
    let mut y = vec![];
    for _ in 0..q {
        input! {
            d_i: usize,
            y_i: [Usize1; d_i],
        }
        y.push(y_i);
    }

    for y_i in y {
        let mut max = None;
        for (i, (x_i, a_i)) in x.iter().zip(a.iter().copied()).enumerate() {
            let mut ok = true;
            for y_ij in y_i.iter() {
                if x_i.contains(y_ij) {
                    ok = false;
                    break;
                }
            }
            if !ok {
                continue;
            }
            match max {
                None => max = Some((a_i, i)),
                Some((a_max, _)) if a_max < a_i => max = Some((a_i, i)),
                _ => {}
            }
        }
        println!(
            "{}",
            match max {
                Some((_, i)) => (i + 1) as i64,
                None => -1,
            }
        )
    }
}
