use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

fn main() {
    input! {
        n: usize,
        d: usize,
        w: usize,
        mut tx: [(usize, usize); n],
    };

    #[derive(Clone, Debug)]
    struct M;
    impl Monoid for M {
        type S = i64;

        fn identity() -> Self::S {
            0
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            *a.max(b)
        }
    }

    #[derive(Clone)]
    struct F(i64);
    impl MapMonoid for F {
        type M = M;
        type F = F;

        fn identity_map() -> Self::F {
            F(0)
        }

        fn mapping(
            f: &Self::F,
            x: &<Self::M as ac_library::Monoid>::S,
        ) -> <Self::M as ac_library::Monoid>::S {
            f.0 + x
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            F(f.0 + g.0)
        }
    }

    tx.sort_by_key(|&(t, x)| (t, x));

    let mut ans = 0;
    let mut lst = LazySegtree::<F>::new(1_000_000);
    let mut r = 0;
    for l in 0..n {
        let (t_l, x_l) = tx[l];
        while r < n && tx[r].0 < t_l + d {
            let (_, x_r) = tx[r];
            lst.apply_range(x_r.saturating_sub(w)..x_r, F(1));
            r += 1;
        }

        ans = ans.max(lst.all_prod());

        if r == l {
            r += 1;
        } else {
            lst.apply_range(x_l.saturating_sub(w)..x_l, F(-1));
        }
    }

    println!("{}", ans);
}
