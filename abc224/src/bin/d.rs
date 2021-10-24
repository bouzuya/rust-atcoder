use std::collections::{HashMap, VecDeque};

use proconio::{input, marker::Usize1};

fn main() {
    input! {
        m: usize,
        uv: [(Usize1, Usize1); m],
        p: [Usize1; 8]
    };
    let mut e = vec![vec![]; 9];
    for (u, v) in uv {
        e[u].push(v);
        e[v].push(u);
    }

    let mut s = vec!['9'; 9];
    for (i, j) in p.into_iter().enumerate() {
        s[j] = (i as u8 + 1 + b'0') as char;
    }
    let s = s.into_iter().collect::<String>();
    let mut dp = HashMap::new();
    dp.insert(s.clone(), 0);
    let mut q = VecDeque::new();
    q.push_back((s, 0));
    while let Some((s, d)) = q.pop_front() {
        if let Some(&d_) = dp.get(s.as_str()) {
            if d > d_ {
                continue;
            }
        }

        let cs = s.chars().collect::<Vec<char>>();
        let i9 = cs.iter().position(|&c| c == '9').unwrap();
        for i in e[i9].iter().copied() {
            let t = (0..9)
                .map(|k| {
                    if k == i {
                        cs[i9]
                    } else if k == i9 {
                        cs[i]
                    } else {
                        cs[k]
                    }
                })
                .collect::<String>();
            if dp.get(t.as_str()).is_some() {
                continue;
            }

            dp.insert(t.clone(), d + 1);
            q.push_back((t, d + 1));
        }
    }
    let ans = *dp.get("123456789").unwrap_or(&-1_i64);
    println!("{}", ans);
}
