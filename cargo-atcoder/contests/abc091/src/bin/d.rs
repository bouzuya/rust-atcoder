use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        b: [usize; n],
    };
    let mut ans = 0_usize;
    for k in 0..30 {
        let t = 1_usize << k;
        let a = a
            .iter()
            .copied()
            .map(|a_i| a_i % (2 * t))
            .collect::<Vec<usize>>();
        let mut b = b
            .iter()
            .copied()
            .map(|b_i| b_i % (2 * t))
            .collect::<Vec<usize>>();
        b.sort();
        let mut count = 0;
        for a_i in a {
            count += b.lower_bound(&(2 * t - a_i)) - b.lower_bound(&(t.saturating_sub(a_i)))
                + b.lower_bound(&(4 * t - a_i))
                - b.lower_bound(&(3 * t - a_i))
        }
        if count % 2 != 0 {
            ans += t;
        }
    }
    println!("{}", ans);
}
