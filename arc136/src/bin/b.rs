use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        b: [usize; n]
    };

    let mut m_a = HashMap::new();
    for a_i in a.iter().copied() {
        *m_a.entry(a_i).or_insert(0) += 1;
    }
    let mut m_b = HashMap::new();
    for b_i in b.iter().copied() {
        *m_b.entry(b_i).or_insert(0) += 1;
    }
    if m_a != m_b {
        println!("No");
        return;
    }

    if m_a.values().any(|&v| v >= 2) {
        println!("Yes");
        return;
    }

    for (i, b_i) in b.iter().copied().enumerate().take(n - 2) {
        let mut j = i;
        while j < n && a[j] != b_i {
            j += 1;
        }
        if j == n {
            println!("No");
            return;
        }
        while i < j {
            if i + 1 == j {
                a[j - 1..=j + 1].rotate_right(2);
                break;
            } else {
                a[j - 2..=j].rotate_right(1);
                j -= 2;
            }
        }
    }
    let ans = a[n - 2] == b[n - 2] && a[n - 1] == b[n - 1];
    println!("{}", if ans { "Yes" } else { "No" });
}
