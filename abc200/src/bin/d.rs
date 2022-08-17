use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let f = |a1: &[usize], a2: &[usize]| {
        println!("Yes");
        println!(
            "{} {}",
            a1.len(),
            a1.iter()
                .copied()
                .map(|j| format!("{}", j + 1))
                .collect::<Vec<String>>()
                .join(" ")
        );
        println!(
            "{} {}",
            a2.len(),
            a2.iter()
                .copied()
                .map(|j| format!("{}", j + 1))
                .collect::<Vec<String>>()
                .join(" ")
        );
    };
    let a = a.into_iter().map(|a_i| a_i % 200).collect::<Vec<usize>>();

    let mut map = HashMap::new();
    for (i, a_i) in a.iter().copied().enumerate() {
        map.entry(a_i).or_insert_with(Vec::new).push(i);
    }
    for (_, is) in map {
        if is.len() >= 2 {
            f(&[is[0]], &[is[1]]);
            return;
        }
    }

    let mut used = vec![vec![]; 200];
    for (i, a_i) in a.iter().copied().enumerate() {
        if !used[a_i].is_empty() {
            f(&used[a_i], &[i]);
            return;
        }

        let mut next = used.clone();
        for j in 0..200 {
            if used[j].is_empty() {
                continue;
            }
            let x = (j + a_i) % 200;
            if !used[x].is_empty() {
                let mut next_y = vec![];
                next_y.extend(&used[j]);
                next_y.push(i);
                f(&used[x], &next_y);
                return;
            }
            if next[x].is_empty() {
                next[x].extend(&used[j]);
                next[x].push(i);
            } else {
                let mut next_y = vec![];
                next_y.extend(&used[j]);
                next_y.push(i);
                f(&next[x], &next_y);
                return;
            }
        }
        if next[a_i].is_empty() {
            next[a_i].push(i);
        } else {
            f(&next[a_i], &[i]);
            return;
        }
        used = next;
    }
    println!("No");
}
