use ac_library::Dsu;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut abc: [(Usize1, Usize1, i64); m]
    };

    abc.sort_by_key(|&(_, _, c)| c);

    let mut ans = abc.iter().copied().map(|(_, _, c)| c).sum::<i64>();
    let mut dsu = Dsu::new(n);
    for (a, b, c) in abc {
        if dsu.same(a, b) {
            if c < 0 {
                ans -= c;
            }
            continue;
        }
        dsu.merge(a, b);
        ans -= c;
    }

    println!("{}", ans);
}
