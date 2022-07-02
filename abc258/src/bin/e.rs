use std::collections::HashSet;

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        q: usize,
        x: usize,
        w: [usize; n],
        k: [Usize1; q],
    };
    let sum_w = w.iter().sum::<usize>();
    let (offset_c, x) = if sum_w == x {
        println!("{}", n);
        return;
    } else if sum_w < x {
        ((x / sum_w) * n, x % sum_w)
    } else {
        (0, x)
    };
    let w2 = w
        .iter()
        .copied()
        .chain(w.iter().copied())
        .collect::<Vec<usize>>();
    let mut y = vec![0; n];
    let mut r = 0;
    let mut s = 0_usize;
    for l in 0..n {
        while (r < 2 * n) && (s + w2[r] < x) {
            s += w2[r];
            r += 1;
        }
        y[l] = offset_c + r - l + 1;
        if r == l {
            r += 1;
        } else {
            s -= w2[l];
        }
    }

    // FIXME
    // let mut set = HashSet::new();
    // let prev = 0;
    // let mut start = 0;
    // let mut index = y[prev % n];
    // set.insert(prev);
    // let mut z = vec![prev];
    // for _ in 0..n {
    //     let prev = index;
    //     index = (index + y[prev % n]) % n;
    //     if set.contains(&index) {
    //         start = z.iter().position(|z_i| *z_i == prev).unwrap();
    //         break;
    //     } else {
    //         set.insert(index);
    //         z.push(prev);
    //     }
    // }
    // let cycle = z.len() - start;
    // for k_i in k {
    //     if k_i <= start {
    //         println!("{}", y[z[k_i % z.len()]]);
    //     } else {
    //         println!("{}", y[z[(start + (k_i - start) % cycle) % z.len()]]);
    //     }
    // }
}
