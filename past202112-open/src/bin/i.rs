use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(usize, usize, usize); m],
    };
    let floor_to_index = abc
        .iter()
        .copied()
        .map(|(a, _, _)| a)
        .chain(abc.iter().copied().map(|(_, b, _)| b))
        .chain(std::iter::once(1))
        .chain(std::iter::once(n))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .enumerate()
        .fold(BTreeMap::new(), |mut m, (i, k)| {
            m.insert(k, i);
            m
        });
    let mut index_to_floor = vec![0; floor_to_index.len()];
    for (&floor, &index) in floor_to_index.iter() {
        index_to_floor[index] = floor;
    }

    let f = |u: usize, v: usize| {
        if u > v {
            u - v
        } else {
            v - u
        }
    };
    let mut edges = vec![BTreeMap::new(); floor_to_index.len()];
    for (a, b, c) in abc {
        let u = *floor_to_index.get(&a).unwrap();
        let v = *floor_to_index.get(&b).unwrap();
        let entry = edges[u].entry(v).or_insert(c);
        *entry = (*entry).min(c).min(f(a, b));
        let entry = edges[v].entry(u).or_insert(c);
        *entry = (*entry).min(c).min(f(a, b));
    }

    for u in 0..floor_to_index.len() {
        let floor_u = index_to_floor[u];
        if u > 0 {
            let v = u - 1;
            let floor_v = index_to_floor[v];
            let d = f(floor_u, floor_v);
            let entry = edges[u].entry(v).or_insert(d);
            *entry = (*entry).min(d);
        }
        if u + 1 < floor_to_index.len() {
            let v = u + 1;
            let floor_v = index_to_floor[v];
            let d = f(floor_u, floor_v);
            let entry = edges[u].entry(v).or_insert(d);
            *entry = (*entry).min(d);
        }
    }
    let inf = 1 << 60;
    let mut dist = vec![inf; floor_to_index.len()];
    let mut pq = BinaryHeap::new();
    pq.push((Reverse(0), *floor_to_index.get(&1).unwrap()));
    dist[*floor_to_index.get(&1).unwrap()] = 0;
    while let Some((Reverse(d_u), u)) = pq.pop() {
        if dist[u] != d_u {
            continue;
        }
        for (&v, &d_v) in edges[u].iter() {
            if v == u {
                continue;
            }

            let d_v = d_u + d_v;
            if d_v < dist[v] {
                dist[v] = d_v;
                pq.push((Reverse(d_v), v));
            };
        }
    }

    let ans = dist[*floor_to_index.get(&n).unwrap()];
    println!("{}", ans);
}
