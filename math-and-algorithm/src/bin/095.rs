use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        cp: [(usize, usize); n],
        q: usize,
        lr: [(Usize1, Usize1); q],
    };
    let s1 = std::iter::once(0)
        .chain(cp.iter().scan(0, |acc, &(c, p)| {
            *acc += match c {
                1 => p,
                2 => 0,
                _ => unreachable!(),
            };
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let s2 = std::iter::once(0)
        .chain(cp.iter().scan(0, |acc, &(c, p)| {
            *acc += match c {
                1 => 0,
                2 => p,
                _ => unreachable!(),
            };
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    for (l, r) in lr {
        println!("{} {}", s1[r + 1] - s1[l], s2[r + 1] - s2[l]);
    }
}
