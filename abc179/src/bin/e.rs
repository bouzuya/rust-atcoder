use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        m: usize,
    };
    if n <= 1_000_000 {
        let mut a = vec![];
        a.push(0);
        a.push(x);
        for i in 2..=n {
            let p = a[i - 1];
            let c = (p * p) % m;
            a.push(c);
        }
        let ans = a.iter().sum::<usize>();
        println!("{}", ans);
        return;
    }

    let mut a = vec![];
    a.push(0);
    a.push(x);
    let mut prefix_len = 0;
    let mut cycle_len = 0;
    let mut map = BTreeMap::new();
    map.insert(x, 1);
    for i in 2.. {
        let p = a[i - 1];
        let c = (p * p) % m;
        match map.get(&c) {
            Some(&j) => {
                prefix_len = j;
                cycle_len = i - j;
                break;
            }
            None => {
                map.insert(c, i);
            }
        }
        a.push(c);
    }

    let n = n - prefix_len;
    let cycle_count = n / cycle_len;
    let prefix_sum = a.iter().copied().take(prefix_len).sum::<usize>();
    let cycle_sum = a.iter().copied().skip(prefix_len).sum::<usize>();
    let suffix_sum = a
        .iter()
        .copied()
        .skip(prefix_len)
        .take(n % cycle_len + 1)
        .sum::<usize>();
    let ans = prefix_sum + cycle_count * cycle_sum + suffix_sum;
    println!("{}", ans);
}
