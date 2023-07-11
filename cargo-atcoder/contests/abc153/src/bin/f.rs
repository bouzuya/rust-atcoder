use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: usize,
        mut xh: [(usize, usize); n],
    };
    xh.sort();

    let mut counts = vec![0_usize; xh.len() + 1];
    let mut count = 0_usize;
    for (i, (x, h)) in xh.iter().copied().enumerate() {
        count -= counts[i];
        if count * a < h {
            let c = ((h - (count * a)) + a - 1) / a;
            count += c;
            counts[xh.upper_bound_by_key(&(x + 2 * d), |(x, _)| *x)] += c;
        }
    }
    println!("{}", counts.iter().sum::<usize>());
}
