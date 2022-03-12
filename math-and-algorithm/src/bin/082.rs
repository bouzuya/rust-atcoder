use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    };
    lr.sort_by_key(|&(_, r)| r);
    let mut count = 1;
    let mut p = lr[0].1;
    for (l, r) in lr.into_iter().skip(1) {
        if l >= p {
            count += 1;
            p = r;
        }
    }
    let ans = count;
    println!("{}", ans);
}
