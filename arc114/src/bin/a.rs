use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize; n],
    };

    let mut ps = vec![];
    for x in 2..=50 {
        if ps.iter().all(|p_i| x % p_i != 0) {
            ps.push(x);
        }
    }
    assert_eq!(ps.len(), 15);

    let mut ans = ps.iter().product::<usize>();
    for bits in 0..1 << ps.len() {
        let mut qs = vec![];
        for i in 0..ps.len() {
            if (bits >> i) & 1 == 1 {
                qs.push(ps[i]);
            }
        }

        if x.iter()
            .copied()
            .all(|x_i| qs.iter().copied().any(|q_i| x_i % q_i == 0))
        {
            ans = cmp::min(ans, qs.iter().product::<usize>());
        }
    }
    println!("{}", ans);
}
