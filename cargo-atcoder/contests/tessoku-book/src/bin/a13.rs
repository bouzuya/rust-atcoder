use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let mut count = 0_usize;
    for (i, a_i) in a.iter().copied().enumerate() {
        count += a.upper_bound(&(a_i + k)) - i - 1;
    }
    let ans = count;
    println!("{}", ans);
}
