use proconio::input;

fn main() {
    input! {
        n: usize,
        mut lr: [(usize, usize); n],
    };
    lr.sort_by_key(|&(l, r)| (r, l));
    let mut count = 1_usize;
    let mut prev = lr[0];
    for (l, r) in lr.into_iter().skip(1) {
        if prev.1 <= l {
            count += 1;
            prev = (l, r);
        }
    }
    let ans = count;
    println!("{}", ans);
}
