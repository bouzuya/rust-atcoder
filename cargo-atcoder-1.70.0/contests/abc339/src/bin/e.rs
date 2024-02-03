use ac_library::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        a: [usize; n],
    };

    let mut segtree = Segtree::<Max<usize>>::new(500_001);
    for a_i in a {
        let max = segtree.prod(a_i.saturating_sub(d)..=(a_i + d).min(500_000));
        let v = segtree.get(a_i).max(max + 1);
        segtree.set(a_i, v);
    }
    let max = segtree.all_prod();
    let ans = if max == 0 { 1 } else { max };
    println!("{}", ans);
}
