use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };

    let mut set1 = HashSet::new();
    for bits in 0..1 << (n / 2) {
        let sum = (0..n / 2)
            .map(|i| if (bits >> i) & 1 == 1 { a[i] } else { 0 })
            .sum::<usize>();
        if sum <= k {
            set1.insert(sum);
        }
    }
    let mut set2 = HashSet::new();
    for bits in 0..1 << (n - (n / 2)) {
        let sum = (0..(n - (n / 2)))
            .map(|i| {
                if (bits >> i) & 1 == 1 {
                    a[n / 2 + i]
                } else {
                    0
                }
            })
            .sum::<usize>();
        if sum <= k {
            set2.insert(sum);
        }
    }

    for x in set1 {
        if set2.contains(&(k - x)) {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
