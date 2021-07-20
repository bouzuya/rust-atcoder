use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(i64, char); 2 * n],
    };
    let f = |ac: &[(i64, char)], c: char| {
        ac.iter()
            .filter(|(_, c_i)| *c_i == c)
            .map(|(a_i, _)| *a_i)
            .collect::<Vec<i64>>()
    };
    let r = f(&ac, 'R');
    let g = f(&ac, 'G');
    let b = f(&ac, 'B');
    let (mut o1, mut o2, mut e) = match (r.len() % 2 == 0, g.len() % 2 == 0, b.len() % 2 == 0) {
        (true, true, true) => {
            println!("0");
            return;
        }
        (true, false, false) => (g, b, r),
        (false, true, false) => (r, b, g),
        (false, false, true) => (r, g, b),
        (true, true, false) | (true, false, true) | (false, true, true) | (false, false, false) => {
            unreachable!()
        }
    };
    o1.sort();
    o2.sort();
    e.sort();

    // o1 + o2
    // o1 + e & o2 + e

    let g = |a1: &[i64], a2: &[i64]| {
        let mut min = 1_000_000_000_000_000_000_i64;
        let mut i1 = 0;
        let mut i2 = 0;
        while i1 < a1.len() && i2 < a2.len() {
            if a1[i1] < a2[i2] {
                min = cmp::min(min, a2[i2] - a1[i1]);
                if i1 + 1 < a1.len() {
                    i1 += 1;
                } else {
                    i2 += 1;
                }
            } else {
                min = cmp::min(min, a1[i1] - a2[i2]);
                if i2 + 1 < a2.len() {
                    i2 += 1;
                } else {
                    i1 += 1;
                }
            }
        }
        min
    };
    let ans = cmp::min(g(&o1, &o2), g(&o1, &e) + g(&o2, &e));
    println!("{}", ans);
}
