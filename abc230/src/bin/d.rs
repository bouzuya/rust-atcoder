use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        mut lr: [(usize, usize); n],
    };
    lr.sort_by_key(|&(a, b)| (b, a));
    let mut ans = 1;
    let mut p = lr[0].1;
    for (l, r) in lr.into_iter().skip(1) {
        if p + d - 1 < l {
            ans += 1;
            p = r;
        }
    }
    println!("{}", ans);
}
