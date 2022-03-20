use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        s: [char; 3],
        t: [char; 3],
    };
    let v: Vec<Vec<usize>> = vec![
        vec![0, 1, 2],
        vec![0, 2, 1],
        vec![1, 0, 2],
        vec![1, 2, 0],
        vec![2, 0, 1],
        vec![2, 1, 0],
    ];
    let mut e = vec![vec![]; v.len()];
    for (i, v_i) in v.iter().enumerate() {
        e[i].push(
            v.iter()
                .position(|v_j| v_j == &vec![v_i[0], v_i[2], v_i[1]])
                .unwrap(),
        );
        e[i].push(
            v.iter()
                .position(|v_j| v_j == &vec![v_i[1], v_i[0], v_i[2]])
                .unwrap(),
        );
        e[i].push(
            v.iter()
                .position(|v_j| v_j == &vec![v_i[2], v_i[1], v_i[0]])
                .unwrap(),
        );
    }

    let s = s
        .into_iter()
        .map(|s_i| match s_i {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        })
        .collect::<Vec<usize>>();
    let t = t
        .into_iter()
        .map(|s_i| match s_i {
            'R' => 0,
            'G' => 1,
            'B' => 2,
            _ => unreachable!(),
        })
        .collect::<Vec<usize>>();

    let s = v.iter().position(|x| x == &s).unwrap();
    let t = v.iter().position(|x| x == &t).unwrap();

    let mut deque = VecDeque::new();
    let inf = 1000;
    let mut dist = vec![inf; v.len()];
    deque.push_back((s, 0));
    dist[s] = 0;
    while let Some((u, d)) = deque.pop_front() {
        for v in e[u].iter().copied() {
            if dist[v] != inf {
                continue;
            }
            dist[v] = d + 1;
            deque.push_back((v, d + 1));
        }
    }
    let ans = (10_usize.pow(18) - dist[t]) % 2 == 0;
    println!("{}", if ans { "Yes" } else { "No" });
}
