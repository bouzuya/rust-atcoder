use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String, String); n],
    };

    let mut used = BTreeSet::new();
    for i in 0..n {
        let (s_i, t_i) = &st[i];

        // s[i] を a_i とできるか
        let mut ok = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            let (s_j, t_j) = &st[j];

            if s_i == s_j || s_i == t_j {
                ok = false;
                break;
            }
        }
        if ok && used.insert(s_i) {
            continue;
        }

        let mut ok = true;
        for j in 0..n {
            if i == j {
                continue;
            }
            let (s_j, t_j) = &st[j];

            if t_i == s_j || t_i == t_j {
                ok = false;
                break;
            }
        }
        if ok && used.insert(t_i) {
            continue;
        }

        println!("No");
        return;
    }
    println!("Yes");
}
