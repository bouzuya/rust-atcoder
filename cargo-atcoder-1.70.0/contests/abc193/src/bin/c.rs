use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut set = HashSet::new();
    for a in 2_usize.. {
        let mut ok = false;
        for b in 2_u32.. {
            match a.checked_pow(b) {
                Some(x) if x <= n => {
                    set.insert(x);
                    ok = true;
                }
                Some(_) | None => {
                    break;
                }
            }
        }
        if !ok {
            break;
        }
    }

    let ans = n - set.len();
    println!("{}", ans);
}
