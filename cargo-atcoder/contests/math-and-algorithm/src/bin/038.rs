use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    };
    let s = std::iter::once(0)
        .chain(a.iter().scan(0, |acc, &i| {
            *acc += i;
            Some(*acc)
        }))
        .collect::<Vec<usize>>();
    for (l, r) in lr {
        let ans = s[r] - s[l - 1];
        println!("{}", ans);
    }
}
