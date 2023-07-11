use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q]
    };
    let c = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();

    for (l, r) in lr {
        let ok = c[r] - c[l - 1];
        let ng = r + 1 - l - ok;
        let ans = match ok.cmp(&ng) {
            std::cmp::Ordering::Less => "lose",
            std::cmp::Ordering::Equal => "draw",
            std::cmp::Ordering::Greater => "win",
        };
        println!("{}", ans);
    }
}
