use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    };
    a.sort();
    b.sort();
    c.sort();

    let mut ans = 0_usize;
    for b_i in b {
        let count_a = a.upper_bound(&(b_i - 1));
        let count_c = n - c.lower_bound(&(b_i + 1));
        ans += count_a * count_c
    }
    println!("{}", ans);
}
