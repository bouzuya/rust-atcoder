use self::atcoder::segtree::{Max, Segtree};
use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    };
    let max_a = 300_000_usize;
    let mut segtree: Segtree<Max<_>> = vec![0; max_a + 1].into();
    for &a_i in a.iter() {
        let l = a_i.saturating_sub(k);
        let r = std::cmp::min(a_i + k, max_a);
        let x = segtree.prod(l, r + 1);
        segtree.set(a_i, x + 1);
    }
    let ans = segtree.prod(0, max_a + 1);
    println!("{}", ans);
}

pub mod atcoder {
    pub mod internal_bit {
        // https://raw.githubusercontent.com/rust-lang-ja/ac-library-rs/ad148b05b07e3ac59cbc2987b8834742ef3d497a/src/internal_bit.rs
        // Skipped:
        //
        // - `bsf` = `__builtin_ctz`: is equivalent to `{integer}::trailing_zeros`

        #[allow(dead_code)]
        pub(crate) fn ceil_pow2(n: u32) -> u32 {
            32 - n.saturating_sub(1).leading_zeros()
        }
    }

    pub mod internal_type_traits {
        // https://raw.githubusercontent.com/rust-lang-ja/ac-library-rs/ad148b05b07e3ac59cbc2987b8834742ef3d497a/src/internal_type_traits.rs
        use std::{
            fmt,
            iter::{Product, Sum},
            ops::{
                Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign,
                Div, DivAssign, Mul, MulAssign, Not, Rem, RemAssign, Shl, ShlAssign, Shr,
                ShrAssign, Sub, SubAssign,
            },
        };

        // Skipped:
        //
        // - `is_signed_int_t<T>`   (probably won't be used directly in `modint.rs`)
        // - `is_unsigned_int_t<T>` (probably won't be used directly in `modint.rs`)
        // - `to_unsigned_t<T>`     (not used in `fenwicktree.rs`)

        /// Corresponds to `std::is_integral` in C++.
        // We will remove unnecessary bounds later.
        //
        // Maybe we should rename this to `PrimitiveInteger` or something, as it probably won't be used in the
        // same way as the original ACL.
        pub trait Integral:
            'static
            + Send
            + Sync
            + Copy
            + Ord
            + Not<Output = Self>
            + Add<Output = Self>
            + Sub<Output = Self>
            + Mul<Output = Self>
            + Div<Output = Self>
            + Rem<Output = Self>
            + AddAssign
            + SubAssign
            + MulAssign
            + DivAssign
            + RemAssign
            + Sum
            + Product
            + BitOr<Output = Self>
            + BitAnd<Output = Self>
            + BitXor<Output = Self>
            + BitOrAssign
            + BitAndAssign
            + BitXorAssign
            + Shl<Output = Self>
            + Shr<Output = Self>
            + ShlAssign
            + ShrAssign
            + fmt::Display
            + fmt::Debug
            + fmt::Binary
            + fmt::Octal
            + Zero
            + One
            + BoundedBelow
            + BoundedAbove
        {
        }

        /// Class that has additive identity element
        pub trait Zero {
            /// The additive identity element
            fn zero() -> Self;
        }

        /// Class that has multiplicative identity element
        pub trait One {
            /// The multiplicative identity element
            fn one() -> Self;
        }

        pub trait BoundedBelow {
            fn min_value() -> Self;
        }

        pub trait BoundedAbove {
            fn max_value() -> Self;
        }

        macro_rules! impl_integral {
            ($($ty:ty),*) => {
                $(
                    impl Zero for $ty {
                        #[inline]
                        fn zero() -> Self {
                            0
                        }
                    }
                    impl One for $ty {
                        #[inline]
                        fn one() -> Self {
                            1
                        }
                    }
                    impl BoundedBelow for $ty {
                        #[inline]
                        fn min_value() -> Self {
                            Self::min_value()
                        }
                    }
                    impl BoundedAbove for $ty {
                        #[inline]
                        fn max_value() -> Self {
                            Self::max_value()
                        }
                    }
                    impl Integral for $ty {}
                )*
            };
        }

        impl_integral!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize);
    }

    pub mod segtree {
        // https://raw.githubusercontent.com/rust-lang-ja/ac-library-rs/ad148b05b07e3ac59cbc2987b8834742ef3d497a/src/segtree.rs
        use super::internal_bit::ceil_pow2;
        use super::internal_type_traits::{BoundedAbove, BoundedBelow, One, Zero};
        use std::cmp::{max, min};
        use std::convert::Infallible;
        use std::marker::PhantomData;
        use std::ops::{Add, Mul};

        // TODO Should I split monoid-related traits to another module?
        pub trait Monoid {
            type S: Clone;
            fn identity() -> Self::S;
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S;
        }

        pub struct Max<S>(Infallible, PhantomData<fn() -> S>);
        impl<S> Monoid for Max<S>
        where
            S: Copy + Ord + BoundedBelow,
        {
            type S = S;
            fn identity() -> Self::S {
                S::min_value()
            }
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                max(*a, *b)
            }
        }

        pub struct Min<S>(Infallible, PhantomData<fn() -> S>);
        impl<S> Monoid for Min<S>
        where
            S: Copy + Ord + BoundedAbove,
        {
            type S = S;
            fn identity() -> Self::S {
                S::max_value()
            }
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                min(*a, *b)
            }
        }

        pub struct Additive<S>(Infallible, PhantomData<fn() -> S>);
        impl<S> Monoid for Additive<S>
        where
            S: Copy + Add<Output = S> + Zero,
        {
            type S = S;
            fn identity() -> Self::S {
                S::zero()
            }
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                *a + *b
            }
        }

        pub struct Multiplicative<S>(Infallible, PhantomData<fn() -> S>);
        impl<S> Monoid for Multiplicative<S>
        where
            S: Copy + Mul<Output = S> + One,
        {
            type S = S;
            fn identity() -> Self::S {
                S::one()
            }
            fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
                *a * *b
            }
        }

        impl<M: Monoid> Default for Segtree<M> {
            fn default() -> Self {
                Segtree::new(0)
            }
        }
        impl<M: Monoid> Segtree<M> {
            pub fn new(n: usize) -> Segtree<M> {
                vec![M::identity(); n].into()
            }
        }
        impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
            fn from(v: Vec<M::S>) -> Self {
                let n = v.len();
                let log = ceil_pow2(n as u32) as usize;
                let size = 1 << log;
                let mut d = vec![M::identity(); 2 * size];
                d[size..(size + n)].clone_from_slice(&v);
                let mut ret = Segtree { n, size, log, d };
                for i in (1..size).rev() {
                    ret.update(i);
                }
                ret
            }
        }
        impl<M: Monoid> Segtree<M> {
            pub fn set(&mut self, mut p: usize, x: M::S) {
                assert!(p < self.n);
                p += self.size;
                self.d[p] = x;
                for i in 1..=self.log {
                    self.update(p >> i);
                }
            }

            pub fn get(&self, p: usize) -> M::S {
                assert!(p < self.n);
                self.d[p + self.size].clone()
            }

            pub fn prod(&self, mut l: usize, mut r: usize) -> M::S {
                assert!(l <= r && r <= self.n);
                let mut sml = M::identity();
                let mut smr = M::identity();
                l += self.size;
                r += self.size;
                while l < r {
                    if l & 1 != 0 {
                        sml = M::binary_operation(&sml, &self.d[l]);
                        l += 1;
                    }
                    if r & 1 != 0 {
                        r -= 1;
                        smr = M::binary_operation(&self.d[r], &smr);
                    }
                    l >>= 1;
                    r >>= 1;
                }
                M::binary_operation(&sml, &smr)
            }
            pub fn all_prod(&self) -> M::S {
                self.d[1].clone()
            }
            pub fn max_right<F>(&self, mut l: usize, f: F) -> usize
            where
                F: Fn(&M::S) -> bool,
            {
                assert!(l <= self.n);
                assert!(f(&M::identity()));
                if l == self.n {
                    return self.n;
                }
                l += self.size;
                let mut sm = M::identity();
                while {
                    // do
                    while l % 2 == 0 {
                        l >>= 1;
                    }
                    if !f(&M::binary_operation(&sm, &self.d[l])) {
                        while l < self.size {
                            l *= 2;
                            let res = M::binary_operation(&sm, &self.d[l]);
                            if f(&res) {
                                sm = res;
                                l += 1;
                            }
                        }
                        return l - self.size;
                    }
                    sm = M::binary_operation(&sm, &self.d[l]);
                    l += 1;
                    // while
                    {
                        let l = l as isize;
                        (l & -l) != l
                    }
                } {}
                self.n
            }
            pub fn min_left<F>(&self, mut r: usize, f: F) -> usize
            where
                F: Fn(&M::S) -> bool,
            {
                assert!(r <= self.n);
                assert!(f(&M::identity()));
                if r == 0 {
                    return 0;
                }
                r += self.size;
                let mut sm = M::identity();
                while {
                    // do
                    r -= 1;
                    while r > 1 && r % 2 == 1 {
                        r >>= 1;
                    }
                    if !f(&M::binary_operation(&self.d[r], &sm)) {
                        while r < self.size {
                            r = 2 * r + 1;
                            let res = M::binary_operation(&self.d[r], &sm);
                            if f(&res) {
                                sm = res;
                                r -= 1;
                            }
                        }
                        return r + 1 - self.size;
                    }
                    sm = M::binary_operation(&self.d[r], &sm);
                    // while
                    {
                        let r = r as isize;
                        (r & -r) != r
                    }
                } {}
                0
            }
            fn update(&mut self, k: usize) {
                self.d[k] = M::binary_operation(&self.d[2 * k], &self.d[2 * k + 1]);
            }
        }
        // Maybe we can use this someday
        // ```
        // for i in 0..=self.log {
        //     for j in 0..1 << i {
        //         print!("{}\t", self.d[(1 << i) + j]);
        //     }
        //     println!();
        // }
        // ```
        pub struct Segtree<M>
        where
            M: Monoid,
        {
            // variable name is _n in original library
            n: usize,
            size: usize,
            log: usize,
            d: Vec<M::S>,
        }
    }
}
