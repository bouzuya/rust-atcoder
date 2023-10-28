use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize; n],
    };
    a.sort();
    let mut max = 0;
    for (i, a_i) in a.iter().copied().enumerate() {
        let count = a[i..].lower_bound(&(a_i + m));
        max = max.max(count);
    }
    let ans = max;
    println!("{}", ans);
}
