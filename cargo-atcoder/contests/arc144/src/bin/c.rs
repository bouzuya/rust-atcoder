use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    };

    if n < 2 * k {
        println!("-1");
        return;
    }

    let mut unused = (1..=n).collect::<BTreeSet<_>>();
    let mut ans = vec![];
    for i in 1..=n {
        let j = i + k;
        if j <= n && unused.contains(&j) && j > n - k {
            unused.remove(&j);
            ans.push(j);
        } else {
            let a = i.saturating_sub(k);
            let x = if 1 <= a {
                *match unused.range(1..=a).next() {
                    Some(x) => x,
                    None => match unused.range(i + k..).next() {
                        Some(x) => x,
                        None => unreachable!(),
                    },
                }
            } else {
                *match unused.range(i + k..).next() {
                    Some(x) => x,
                    None => unreachable!(),
                }
            };
            unused.remove(&x);
            ans.push(x);
        }
    }

    for a in ans {
        println!("{}", a);
    }
}
