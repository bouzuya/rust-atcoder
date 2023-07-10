use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        b: [usize; n],
        mut c: [usize; n],
    }
    a.sort();
    c.sort();

    let mut sum = 0_usize;
    for b_i in b {
        let count_a = a.lower_bound(&b_i);
        let count_b = n - c.lower_bound(&(b_i + 1));
        sum += count_a * count_b;
    }
    let ans = sum;
    println!("{}", ans);
}
