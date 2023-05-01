use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        w: [usize; n],
    }

    let mut count1 = HashMap::new();
    for bits in 0..1 << (n / 2) {
        let mut sum = 0_usize;
        for i in 0..n / 2 {
            if (bits >> i) & 1 == 1 {
                sum += w[i];
            }
        }
        *count1.entry(sum).or_insert(0) += 1_usize;
    }

    let mut count2 = HashMap::new();
    for bits in 0..1 << (n - (n / 2)) {
        let mut sum = 0_usize;
        for i in 0..(n - (n / 2)) {
            if (bits >> i) & 1 == 1 {
                sum += w[n / 2 + i];
            }
        }
        *count2.entry(sum).or_insert(0) += 1_usize;
    }

    let mut ans = 0_usize;
    for (sum, c1) in count1 {
        let c2 = match x.cmp(&sum) {
            std::cmp::Ordering::Less => 0_usize,
            std::cmp::Ordering::Equal => 1_usize,
            std::cmp::Ordering::Greater => *count2.get(&(x - sum)).unwrap_or(&0),
        };
        ans += c1 * c2;
    }
    println!("{}", ans);
}
