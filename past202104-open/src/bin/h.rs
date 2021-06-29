use std::cmp;

use proconio::input;

fn cumsum(a: &[usize]) -> Vec<usize> {
    std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect()
}

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
        q: usize,
        pt: [(usize, usize); n],
    };
    let a = {
        let mut a = pt
            .iter()
            .filter(|(_, t_i)| *t_i == 0)
            .map(|(p_i, _)| *p_i)
            .collect::<Vec<usize>>();
        a.sort();
        a.into_iter().take(m).collect::<Vec<usize>>()
    };
    let b = {
        let mut b = pt
            .iter()
            .filter(|(_, t_i)| *t_i == 1)
            .map(|(p_i, _)| *p_i)
            .collect::<Vec<usize>>();
        b.sort();
        b.into_iter().take(m).collect::<Vec<usize>>()
    };

    let min_i = if a.len() < m { m - a.len() } else { 0 };
    let mut s_a = cumsum(&a);
    for i in a.len()..m {
        s_a.push(s_a[i] + b[i - a.len()]);
    }
    let mut s_b = cumsum(&b);
    while s_b.len() <= m {
        s_b.push(1_000_000_000_000_000);
    }
    let mut min = 1_000_000_000_000_000;
    for i in min_i..=m {
        min = cmp::min(min, s_a[m - i] + s_b[i] + (i + k - 1) / k * q);
    }

    let ans = min;
    println!("{}", ans);
}
