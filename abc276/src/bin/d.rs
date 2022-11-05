use std::collections::{HashMap, HashSet};

use proconio::input;

fn prime_factorization(n: usize) -> HashMap<usize, usize> {
    let mut p = HashMap::new();
    if n < 2 {
        return p;
    }
    let mut n = n; // shadowing
    for i in 2.. {
        if i * i > n {
            break;
        }
        let mut c = 0;
        while n % i == 0 {
            c += 1;
            n /= i;
        }
        if c > 0 {
            p.insert(i, c);
        }
    }
    if n != 1 {
        p.insert(n, 1);
    }
    p
}

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    };

    let mut ok = true;
    for i in 1..n {
        if a[i] != a[0] {
            ok = false;
        }
    }
    if ok {
        println!("0");
        return;
    }

    let mut maps = vec![];
    for a_i in a.iter().copied() {
        let map = prime_factorization(a_i);
        maps.push(map);
    }

    let mut ok = true;
    let mut map_a_0 = prime_factorization(a[0]);
    map_a_0.remove(&2);
    map_a_0.remove(&3);
    let set_a_0 = map_a_0.keys().collect::<HashSet<_>>();
    let mut min_2 = 1_000_000_000;
    let mut min_3 = 1_000_000_000;
    for m in maps.iter() {
        let mut map = m.clone();
        min_2 = min_2.min(*m.get(&2).unwrap_or(&0));
        min_3 = min_3.min(*m.get(&3).unwrap_or(&0));
        map.remove(&2);
        map.remove(&3);
        let set = map.keys().collect::<HashSet<_>>();
        if set_a_0.len() != set.len() {
            ok = false;
            break;
        }
        if set_a_0 != set {
            ok = false;
            break;
        }
    }
    if !ok {
        println!("-1");
        return;
    }

    let mut count = 0_usize;
    for m in maps.iter() {
        count += *m.get(&2).unwrap_or(&0) - min_2;
        count += *m.get(&3).unwrap_or(&0) - min_3;
    }

    let ans = count;
    println!("{}", ans);
}
