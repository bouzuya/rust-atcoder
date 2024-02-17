use ac_library::{lazysegtree::LazySegtree, Additive, MapMonoid, Monoid, Segtree};
use proconio::{
    input,
    marker::{Chars, Usize1},
};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        query: [(usize, Usize1, usize); q],
    };

    if n == 1 {
        for (t, _, _) in query {
            match t {
                1 => {}
                2 => println!("Yes"),
                _ => unreachable!(),
            }
        }
        return;
    }

    #[derive(Clone, Copy)]
    struct M(usize);
    impl Monoid for M {
        type S = usize;

        fn identity() -> Self::S {
            0
        }

        fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
            (a + b) % 2
        }
    }

    #[derive(Clone, Copy)]
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
            if f.0 == 0 {
                *x
            } else {
                1 - x
            }
        }

        fn composition(f: &Self::F, g: &Self::F) -> Self::F {
            F((f.0 + g.0) % 2)
        }
    }

    let mut st = Segtree::<Additive<usize>>::new(n - 1);
    for i in 0..n - 1 {
        st.set(i, if s[i] == s[i + 1] { 1 } else { 0 });
    }

    let mut lst = LazySegtree::<F>::new(n);
    for i in 0..n {
        lst.set(i, if s[i] == '1' { 1 } else { 0 });
    }
    for (t, l, r) in query {
        match t {
            1 => {
                lst.apply_range(l..r, F(1));
                if l > 0 {
                    if lst.get(l - 1) == lst.get(l) {
                        st.set(l - 1, 1);
                    } else {
                        st.set(l - 1, 0);
                    }
                }
                if l + 1 < n {
                    if lst.get(l) == lst.get(l + 1) {
                        st.set(l, 1);
                    } else {
                        st.set(l, 0);
                    }
                }
                if r > 1 {
                    if lst.get(r - 2) == lst.get(r - 1) {
                        st.set(r - 2, 1);
                    } else {
                        st.set(r - 2, 0);
                    }
                }
                if r < n {
                    if lst.get(r - 1) == lst.get(r) {
                        st.set(r - 1, 1);
                    } else {
                        st.set(r - 1, 0);
                    }
                }
                // println!("l = {} r = {} ", l, r);
                // for i in 0..n {
                //     print!("{} ", lst.get(i));
                // }
                // println!();
                // for i in 0..n - 1 {
                //     print!("{} ", st.get(i));
                // }
                // println!();
            }
            2 => {
                let ans = if r == 1 { st.get(0) } else { st.prod(l..r - 1) } == 0;
                println!("{}", if ans { "Yes" } else { "No" });
            }
            _ => unreachable!(),
        }
    }
}
