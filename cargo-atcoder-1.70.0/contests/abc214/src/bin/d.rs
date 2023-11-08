use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        mut uvw: [(Usize1, Usize1, usize); n - 1],
    };
    uvw.sort_by_key(|&(_, _, w)| w);
    let mut sum = 0_usize;
    let mut dsu = Dsu::new(n);
    for (u, v, w) in uvw {
        sum += dsu.size(u) * dsu.size(v) * w;
        dsu.merge(u, v);
    }
    let ans = sum;
    println!("{}", ans);
}
