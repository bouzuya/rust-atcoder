use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [i64; n],
        mut b: [i64; m],
    };
    b.sort();
    let mut min = (a[0] - b[0]).abs();
    for a_i in a {
        let r = b.lower_bound(&a_i);
        let l = r.saturating_sub(1);
        if let Some(b_r) = b.get(r) {
            min = min.min((b_r - a_i).abs());
        }
        if let Some(b_l) = b.get(l) {
            min = min.min((b_l - a_i).abs());
        }
    }
    let ans = min;
    println!("{}", ans);
}
