use std::collections::{BTreeSet, HashSet, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    let mut t = vec![];
    for _ in 0..n {
        input! {
            k_i: usize,
            t_i: [usize; k_i],
        }
        t.push(t_i);
    }
    input! {
        m: usize,
        a: [usize; m],
    }

    let mut max1 = BTreeSet::new();
    let mut max2 = BTreeSet::new();
    for i in 0..n {
        max1.insert((t[i][0], i, 0));
        if 1 < t[i].len() {
            max2.insert((t[i][1], i, 1));
        }
    }
    let mut used = HashSet::new();
    for a_i in a {
        match a_i {
            1 => {
                let (x, j, k) = max1.pop_last().unwrap();
                used.insert((x, j, k));
                let mut next = VecDeque::new();
                for l in k + 1..t[j].len() {
                    if next.len() == 2 {
                        break;
                    }
                    if used.contains(&(t[j][l], j, l)) {
                        continue;
                    }
                    next.push_back((t[j][l], j, l));
                }
                if let Some(v) = next.pop_front() {
                    max1.insert(v);
                    max2.remove(&v);
                }
                if let Some(v) = next.pop_front() {
                    max2.insert(v);
                }
                println!("{}", x);
            }
            2 => match (max1.pop_last(), max2.pop_last()) {
                (None, None) | (None, Some(_)) => unreachable!(),
                (Some((x, j, k)), None) => {
                    used.insert((x, j, k));
                    let mut next = VecDeque::new();
                    for l in k + 1..t[j].len() {
                        if next.len() == 2 {
                            break;
                        }
                        if used.contains(&(t[j][l], j, l)) {
                            continue;
                        }
                        next.push_back((t[j][l], j, l));
                    }
                    if let Some(v) = next.pop_front() {
                        max1.insert(v);
                        max2.remove(&v);
                    }
                    if let Some(v) = next.pop_front() {
                        max2.insert(v);
                    }
                    println!("{}", x);
                }
                (Some((x1, j1, k1)), Some((x2, j2, k2))) => {
                    if x1 > x2 {
                        max2.insert((x2, j2, k2));
                        let (x, j, k) = (x1, j1, k1);
                        used.insert((x, j, k));
                        let mut next = VecDeque::new();
                        for l in k + 1..t[j].len() {
                            if next.len() == 2 {
                                break;
                            }
                            if used.contains(&(t[j][l], j, l)) {
                                continue;
                            }
                            next.push_back((t[j][l], j, l));
                        }
                        if let Some(v) = next.pop_front() {
                            max1.insert(v);
                            max2.remove(&v);
                        }
                        if let Some(v) = next.pop_front() {
                            max2.insert(v);
                        }
                        println!("{}", x);
                    } else {
                        max1.insert((x1, j1, k1));
                        let (x, j, k) = (x2, j2, k2);
                        used.insert((x, j, k));
                        let mut next = VecDeque::new();
                        for l in k + 1..t[j].len() {
                            if next.len() == 2 {
                                break;
                            }
                            if used.contains(&(t[j][l], j, l)) {
                                continue;
                            }
                            next.push_back((t[j][l], j, l));
                        }
                        if let Some(v) = next.pop_front() {
                            max2.insert(v);
                        }
                        println!("{}", x);
                    }
                }
            },
            _ => unreachable!(),
        }
    }
}
