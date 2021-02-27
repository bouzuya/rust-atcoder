use proconio::input;
use std::collections::BTreeSet;

fn main() {
    input! {
        n: usize,
    };
    let mut set = BTreeSet::new();
    for i in 2.. {
        if i * i > n {
            break;
        }
        for j in 2.. {
            match i.checked_pow(j) {
                None => break,
                Some(x) => {
                    if x > n {
                        break;
                    }
                    set.insert(x);
                }
            }
        }
    }
    let ans = n - set.len();
    println!("{}", ans);
}
