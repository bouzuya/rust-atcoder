use std::collections::{BTreeSet, HashMap};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        h: usize,
        w: usize,
        r_s: Usize1,
        c_s: Usize1,
        n: usize,
        rc: [(Usize1, Usize1); n],
        q: usize,
        dl: [(char, usize); q],
    };

    let mut rs = HashMap::new();
    for (r, c) in rc.iter().copied() {
        rs.entry(r).or_insert_with(BTreeSet::new).insert(c);
    }
    let mut cs = HashMap::new();
    for (r, c) in rc.iter().copied() {
        cs.entry(c).or_insert_with(BTreeSet::new).insert(r);
    }

    let mut cr = r_s;
    let mut cc = c_s;
    for (d, l) in dl {
        match d {
            'L' => {
                let mut nc = if (0..w as i64).contains(&(cc as i64 - l as i64)) {
                    cc - l
                } else {
                    0
                };
                if nc < cc {
                    if let Some(c) = rs
                        .get(&cr)
                        .and_then(|rs_cs| rs_cs.range(nc..cc).rev().next())
                    {
                        nc = *c + 1;
                    }
                }
                cc = nc;
            }
            'R' => {
                let mut nc = if (0..w as i64).contains(&(cc as i64 + l as i64)) {
                    cc + l
                } else {
                    w - 1
                };
                if cc + 1 <= nc {
                    if let Some(c) = rs
                        .get(&cr)
                        .and_then(|rs_cs| rs_cs.range(cc + 1..=nc).next())
                    {
                        nc = *c - 1;
                    }
                }
                cc = nc;
            }
            'U' => {
                let mut nr = if (0..h as i64).contains(&(cr as i64 - l as i64)) {
                    cr - l
                } else {
                    0
                };
                if nr < cr {
                    if let Some(r) = cs
                        .get(&cc)
                        .and_then(|rs_cs| rs_cs.range(nr..cr).rev().next())
                    {
                        nr = *r + 1;
                    }
                }
                cr = nr;
            }
            'D' => {
                let mut nr = if (0..h as i64).contains(&(cr as i64 + l as i64)) {
                    cr + l
                } else {
                    h - 1
                };
                if cr + 1 <= nr {
                    if let Some(r) = cs
                        .get(&cc)
                        .and_then(|rs_cs| rs_cs.range(cr + 1..=nr).next())
                    {
                        nr = *r - 1;
                    }
                }
                cr = nr;
            }
            _ => unreachable!(),
        }

        println!("{} {}", cr + 1, cc + 1);
    }
}
