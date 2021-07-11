use std::collections::{BTreeMap, BTreeSet, VecDeque};

use proconio::{input, marker::Chars};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum R {
    Win,
    Lose,
    Draw,
}

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    };
    let s = s
        .into_iter()
        .map(|s_i| {
            let head = &s_i[0..3];
            let tail = &s_i[s_i.len() - 3..s_i.len()];
            (
                head.iter().collect::<String>(),
                tail.iter().collect::<String>(),
            )
        })
        .collect::<Vec<(String, String)>>();

    let mut set = BTreeSet::new();
    for (head, tail) in s.iter() {
        set.insert(head.as_str());
        set.insert(tail.as_str());
    }
    let mut map = BTreeMap::new();
    for (i, &k) in set.iter().enumerate() {
        map.insert(k, i);
    }

    let mut e_inv = vec![vec![]; map.len()];
    let mut e_count = vec![0; map.len()];
    for (head, tail) in s.iter() {
        let u = map[head.as_str()];
        let v = map[tail.as_str()];
        e_inv[v].push(u);
        e_count[u] += 1;
    }

    let mut r = vec![R::Draw; map.len()];
    let mut q = VecDeque::new();
    for u in 0..map.len() {
        if e_count[u] == 0 {
            r[u] = R::Lose;
            q.push_back(u);
        }
    }
    while let Some(v) = q.pop_front() {
        match r[v] {
            R::Lose => {
                for u in e_inv[v].iter().copied() {
                    if r[u] == R::Draw {
                        r[u] = R::Win;
                        q.push_back(u);
                    }
                }
            }
            R::Win => {
                for u in e_inv[v].iter().copied() {
                    if r[u] == R::Draw {
                        e_count[u] -= 1;
                        if e_count[u] == 0 {
                            r[u] = R::Lose;
                            q.push_back(u);
                        }
                    }
                }
            }
            R::Draw => unreachable!(),
        }
    }

    for (_, tail) in s.iter() {
        println!(
            "{}",
            match r[map[tail.as_str()]] {
                R::Win => "Aoki",
                R::Lose => "Takahashi",
                R::Draw => "Draw",
            }
        );
    }
}
