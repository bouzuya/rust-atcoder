use std::collections::{BTreeMap, HashMap};

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        t: Chars,
        mut s: [Chars; n],
    };

    let mut len1 = vec![0_usize; n];
    for (i, s_i) in s.iter().enumerate() {
        let mut index = 0_usize;
        for (j, t_j) in t.iter().copied().enumerate() {
            if let Some(k) = s_i[index..].iter().position(|s_ij| s_ij == &t_j) {
                index = index + k + 1;
                len1[i] = j + 1;
            } else {
                break;
            }
        }
    }

    let mut len2 = vec![0_usize; n];
    for (i, s_i) in s.iter_mut().enumerate() {
        s_i.reverse();
        let mut index = 0_usize;
        for (j, t_j) in t.iter().copied().rev().enumerate() {
            if let Some(k) = s_i[index..].iter().position(|s_ij| s_ij == &t_j) {
                index = index + k + 1;
                len2[i] = j + 1;
            } else {
                break;
            }
        }
    }

    let mut map1 = HashMap::new();
    for len1_i in len1.iter().copied() {
        *map1.entry(len1_i).or_insert(0_usize) += 1;
    }
    let mut map2 = BTreeMap::new();
    for len2_i in len2.iter().copied() {
        *map2.entry(len2_i).or_insert(0_usize) += 1;
    }
    let mut m2 = vec![0_usize; 1_000_000 + 1];
    for (&k, &count) in map2.iter() {
        m2[k] = count;
    }
    let mut sum = m2
        .iter()
        .rev()
        .scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        })
        .collect::<Vec<usize>>();
    sum.reverse();

    let mut ans = 0_usize;
    for i in 0..=t.len() {
        if let Some(c1) = map1.get(&i) {
            let j = t.len() - i;
            let c2 = sum[j];
            ans += c1 * c2;
        }
    }
    println!("{}", ans);
}
