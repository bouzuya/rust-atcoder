use ac_library::{LazySegtree, MapMonoid, Monoid};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    };

    struct M(usize);

    impl Monoid for M {
        type S = usize;

        fn identity() -> Self::S {
            0_usize
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            a + b
        }
    }

    #[derive(Clone, Copy, Debug)]
    struct F(usize);
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

    let mut lst = LazySegtree::<F>::new(n);
    for (i, a_i) in a.iter().copied().enumerate() {
        lst.set(i, a_i);
    }

    for b_i in b {
        let v = lst.get(b_i);
        lst.set(b_i, 0);
        lst.apply_range(0..n, F(v / n));
        let v = v % n;
        if v > 0 {
            if b_i == n - 1 {
                lst.apply_range(0..v, F(1));
            } else if b_i + 1 + v <= n {
                lst.apply_range(b_i + 1..b_i + 1 + v, F(1));
            } else {
                lst.apply_range(b_i + 1..n, F(1));
                lst.apply_range(0..(b_i + 1 + v) % n, F(1));
            }
        }
    }
    for i in 0..n {
        let x = lst.get(i);
        println!("{}", x);
    }
}
