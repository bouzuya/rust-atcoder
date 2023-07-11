use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    };
    let max_lr = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc = (*acc).max(i);
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    let max_rl = std::iter::once(0)
        .chain(a.iter().rev().scan(0, |acc, &i| {
            *acc = (*acc).max(i);
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    for (l, r) in lr {
        let ans = max_lr[l - 1].max(max_rl[n - r]);
        println!("{}", ans);
    }
}
