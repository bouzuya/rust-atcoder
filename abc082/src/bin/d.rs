use std::collections::BTreeSet;

use proconio::input;
use proconio::marker::Chars;

fn f(fs: &Vec<i64>, s: i64) -> BTreeSet<i64> {
    let mut prev = BTreeSet::new();
    prev.insert(s);
    for &f in fs {
        let mut next = BTreeSet::new();
        for p in prev {
            next.insert(p + f);
            next.insert(p - f);
        }
        prev = next.clone();
    }
    prev
}

fn main() {
    input! {
        s: Chars,
        x: i64,
        y: i64,
    };
    let (xs, ys, init) = {
        let mut xs = vec![];
        let mut ys = vec![];
        let mut first = true;
        let mut init = 0;
        let mut is_x = true;
        let mut count = 0;
        for &s_i in s.iter().chain(std::iter::once(&'T')) {
            match s_i {
                'F' => {
                    if first {
                        init += 1;
                    } else {
                        count += 1;
                    }
                }
                'T' => {
                    if first {
                        first = false;
                    }
                    if count > 0 {
                        if is_x {
                            xs.push(count);
                        } else {
                            ys.push(count);
                        }
                    }
                    is_x = !is_x;
                    count = 0;
                }
                _ => unreachable!(),
            }
        }
        (xs, ys, init)
    };

    let ok_x = f(&xs, init);
    let ok_y = f(&ys, 0);
    let ans = ok_x.contains(&x) && ok_y.contains(&y);
    println!("{}", if ans { "Yes" } else { "No" });
}
