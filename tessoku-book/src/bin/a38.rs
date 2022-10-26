use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lrh: [(usize, usize, usize); n],
    };
    let mut max = vec![24; d + 1];
    for (l, r, h) in lrh {
        for j in l..=r {
            max[j] = max[j].min(h);
        }
    }
    let ans = max.into_iter().skip(1).sum::<usize>();
    println!("{}", ans);
}
